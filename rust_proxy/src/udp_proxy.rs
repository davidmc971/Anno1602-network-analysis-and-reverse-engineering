use std::{error::Error, net::SocketAddr};

use tokio::net::UdpSocket;
use tracing::info;

pub struct UdpServer {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl UdpServer {
    pub async fn new(address: String, buffer_size: usize) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(&address).await?;
        info!("[UDP] Socket Listening on: {}", socket.local_addr()?);

        Ok(Self {
            socket,
            buf: vec![0; buffer_size],
            to_send: None,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let Self {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                let data = buf[..size].as_mut();
                data.reverse();

                let amt = socket.send_to(data, &peer).await?;

                println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}
