use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use tokio::{
    net::UdpSocket,
    sync::{mpsc, oneshot},
};
use tracing::{info, trace};

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
        tx_accumulator: mpsc::Sender<(Message, oneshot::Sender<Message>)>,
    ) -> Result<(), Box<dyn Error>> {
        #[allow(unused)]
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

            trace!("Incoming data: {} bytes", data.len());

            trace!("Received {} bytes from {}", size, peer);

            let target_ip: Ipv4Addr;
            let target_port: u16;

            if peer.ip().cmp(&IpAddr::V4(host_addr)).is_eq() {
                trace!("Message from host.");
                target_ip = Ipv4Addr::new(10, 20, 0, 2);
                target_port = port;
            } else {
                trace!("Message from client.");
                target_ip = Ipv4Addr::new(10, 30, 0, 2);
                target_port = port;
            }
            let target = format!("{}:{}", target_ip, target_port);

            let (once_tx, once_rx) = oneshot::channel();
            let _ = tx_accumulator
                .send((
                    Message {
                        data: data.to_vec(),
                        origin: (peer.ip(), peer.port()),
                        destination: (IpAddr::V4(target_ip), target_port),
                        mask_as_address: None,
                    },
                    once_tx,
                ))
                .await;
            let processed_message = once_rx.await.unwrap();

            let bytes_sent = socket.send_to(&processed_message.data, &target).await?;

            trace!("Sent {} bytes to {}", bytes_sent, target);
        }
    }
}
