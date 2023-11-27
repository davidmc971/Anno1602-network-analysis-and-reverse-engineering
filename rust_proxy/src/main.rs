#![warn(rust_2018_idioms)]
#![allow(unused)]

mod parser;
mod tcp_proxy;
mod udp_proxy;

use std::env;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};

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

            let maybe_dplay_signature = &data[20..24];

            if maybe_dplay_signature == [112, 108, 97, 121] {
                debug!("DPLAY detected, injecting proxy IP");
                if let Some((address, port)) = response.mask_as_address {
                    if address.is_ipv4() {
                        if let IpAddr::V4(address) = address {
                            data[8..12].copy_from_slice(&(address as Ipv4Addr).octets());
                        }
                    }
                }
                //    dplay_proxied_ip = self.dplay_proxied_ip
                //    addr = list(map(int, dplay_proxied_ip.split('.')))
                //    struct.pack_into("<BBBB", data_client, 8, addr[0], addr[1], addr[2], addr[3])
                //    print("[DPLAY, {}] injected proxy ip".format(self.port))
            }

            debug!("Sending: {:?}", response);

            let _ = responder.send(response);
        }
    });

    // for i in 0..2 {
    //     let tx = tx.clone();
    //     let thread_span = span!(Level::DEBUG, "Thread", num = i);
    //     tokio::spawn(async move {
    //         for j in 0..10 {
    //             let (res_tx, res_rx) = oneshot::channel();
    //             let _ = tx.send((format!("Message from thread {}: {}", i, j), res_tx)).await;
    //             let x = res_rx.await;
    //             let entered = thread_span.enter();
    //             debug!("{}", x.unwrap());
    //             drop(entered);
    //         }
    //     });
    // }
    

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
