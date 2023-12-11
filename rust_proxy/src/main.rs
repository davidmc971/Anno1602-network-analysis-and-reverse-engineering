#![warn(rust_2018_idioms)]
#![allow(unused)]

mod parser;
mod tcp_proxy;
mod udp_proxy;

use std::env;
use std::error::Error;
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Shr;


use byteorder::{LittleEndian, ReadBytesExt, BigEndian};
use futures::FutureExt;
use serde::de::Expected;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio::try_join;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::{debug, error, info, Level, span, Span};
use tracing_subscriber::FmtSubscriber;

use crate::tcp_proxy::TcpProxy;
use crate::udp_proxy::UdpProxy;

#[derive(Debug, Clone)]
pub struct Message {
    pub data: Vec<u8>,
    pub origin: (IpAddr, u16),
    pub destination: (IpAddr, u16),
    pub mask_as_address: Option<(IpAddr, u16)>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    {
        // Setting up color_eyre and tracing for debug output

        let _ = color_eyre::install();

        let mut subscriber = FmtSubscriber::builder();
        if !cfg!(debug_assertions) {
            subscriber = subscriber.with_max_level(Level::INFO);
        } else {
            subscriber = subscriber.with_max_level(Level::DEBUG);
        }

        tracing::subscriber::set_global_default(subscriber.finish())
            .expect("setting default subscriber failed");
    }

    let (tx, mut rx) = 
        // mpsc - multiple producer, single consumer + oneshot to send messages back to origin
        mpsc::channel::<(Message, oneshot::Sender<Message>)>(32);

    tokio::spawn(async move {
        let accumulator_span = span!(Level::DEBUG, "Accumulator");
        while let Some((mut message, responder)) = rx.recv().await {
            let mut response = message;

            {
                let entered = accumulator_span.enter();
                debug!("Received: {:?}", response);
                drop(entered);
            }

            let data = response.data.as_mut_slice();

            let maybe_dplay_signature = if data.len() >= 28 {&data[20..24]} else {&[]};

            if maybe_dplay_signature == [112, 108, 97, 121] {
                let mut size_and_token_combined_slice = [0_u8; 4];
                size_and_token_combined_slice.copy_from_slice(&data[0..4]);
                let size_and_token_combined_u32 = u32::from_le_bytes(size_and_token_combined_slice);
                let size_u20 = size_and_token_combined_u32 & 0x000fffff;
                let token_u12 = (size_and_token_combined_u32 & 0xfff00000) >> 20;
                let mut sock_addr_in_cursor = Cursor::new(&data[4..20]);
                let sock_addr_in_address_family = sock_addr_in_cursor.read_u16::<LittleEndian>().unwrap();
                let sock_addr_in_port = sock_addr_in_cursor.read_u16::<BigEndian>().unwrap();
                let sock_addr_in_ip_address = sock_addr_in_cursor.read_u32::<BigEndian>().unwrap();
                let mut version_and_command_cursor = Cursor::new(&data[24..28]);
                // TODO: parse command with enum containing all valid dplay commands
                let command = version_and_command_cursor.read_u16::<LittleEndian>().unwrap();
                let version = version_and_command_cursor.read_u16::<LittleEndian>().unwrap();
                debug!("DPLAY, size: {}, token: 0x{:x}, SockAddr: [AF: 0x{:x}, port: {}, ip_addr: 0x{:x}, signature: play, version: {:?}, command: {:?}]", size_u20, token_u12, sock_addr_in_address_family, sock_addr_in_port, sock_addr_in_ip_address, version, command);
                if let Some((address, port)) = response.mask_as_address {
                    if address.is_ipv4() {
                        if let IpAddr::V4(address) = address {
                            debug!("Injecting proxy IP");
                            data[8..12].copy_from_slice(&(address as Ipv4Addr).octets());
                        }
                    }
                }
            }

            debug!("Sending: {:?}", response);

            let _ = responder.send(response);
        }
    });

    let udp_proxy_session_init = UdpProxy::new("0.0.0.0".to_string(), 47624, 1024).await?;
    let tcp_proxy_session_data = TcpProxy::new().await?;
    let udp_proxy_game_data = UdpProxy::new("0.0.0.0".to_string(), 2350, 1024).await?;

    let res = try_join!(
        udp_proxy_session_init.run(tx.clone()),
        tcp_proxy_session_data.run(tx.clone()),
        udp_proxy_game_data.run(tx.clone()),
    );

    if res.is_err() {
        let err = res.unwrap_err();
        error!("{:?}", err);
        Err(err)
    } else {
        Ok(())
    }
}
