use std::{error::Error, net::SocketAddr};

use tokio::net::UdpSocket;
use tracing::{debug, info};

pub struct UdpServer {
    socket: UdpSocket,
    buf: Vec<u8>,
    incoming: Option<(usize, SocketAddr)>,
}

impl UdpServer {
    pub async fn new(address: String, buffer_size: usize) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(&address).await?;
        info!("[UDP] Socket Listening on: {}", socket.local_addr()?);

        Ok(Self {
            socket,
            buf: vec![0; buffer_size],
            incoming: None,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let Self {
            socket,
            mut buf,
            mut incoming,
        } = self;

        loop {
            if let Some((size, peer)) = incoming {
                let data = buf[..size].as_mut();

                debug!("Incoming data:\n{:?}\n{:x?}", data, data);

                info!("Received {} bytes from {}", size, peer);

                let bytes_sent = socket.send_to(data, "10.30.0.2:47624").await?;

                info!("Sent {} bytes to {}", bytes_sent, "10.30.0.2:47624");
            }

            incoming = Some(socket.recv_from(&mut buf).await?);
        }
    }
}
