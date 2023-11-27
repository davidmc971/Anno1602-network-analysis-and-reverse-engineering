use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use tokio::{
    net::UdpSocket,
    sync::{mpsc, oneshot},
};
use tracing::{debug, info};

use crate::Message;

pub struct UdpProxy {
    socket: UdpSocket,
    buf: Vec<u8>,
    incoming: Option<(usize, SocketAddr)>,
}

impl UdpProxy {
    pub async fn new(
        listen_address: String,
        port: u16,
        buffer_size: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let address = format!("{}:{}", listen_address, port);
        let socket = UdpSocket::bind(&address).await?;
        info!("[UDP] Socket Listening on: {}", socket.local_addr()?);

        Ok(Self {
            socket,
            buf: vec![0; buffer_size],
            incoming: None,
        })
    }

    pub async fn run(
        self,
        tx: mpsc::Sender<(Message, oneshot::Sender<Message>)>,
    ) -> Result<(), Box<dyn Error>> {
        let Self {
            socket,
            mut buf,
            mut incoming,
        } = self;

        let port = socket.local_addr().unwrap().port();
        let host_addr = Ipv4Addr::new(10, 30, 0, 2);

        loop {
            let (size, peer) = socket.recv_from(&mut buf).await?;

            let data = buf[..size].as_mut();

            debug!("Incoming data: {} bytes", data.len());

            info!("Received {} bytes from {}", size, peer);

            let mut bytes_sent = 0;

            let mut target: String;

            if peer.ip().cmp(&IpAddr::V4(host_addr)).is_eq() {
                debug!("Message from host.");
                target = format!("10.20.0.2:{}", port);
                bytes_sent = socket.send_to(data, &target).await?;
            } else {
                debug!("Message from client.");
                target = format!("10.30.0.2:{}", port);
                bytes_sent = socket.send_to(data, &target).await?;
            }

            info!("Sent {} bytes to {}", bytes_sent, target);
        }
    }
}
