use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::{error, info, trace};

use crate::Message;

pub struct TcpProxy {}

impl TcpProxy {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    pub async fn run(
        self,
        tx_accumulator: mpsc::Sender<(Message, oneshot::Sender<Message>)>,
    ) -> Result<(), Box<dyn Error>> {
        let listen_addr = "0.0.0.0:2300";

        let anno_host_ip = IpAddr::V4(Ipv4Addr::new(10, 30, 0, 2));
        #[allow(unused)]
        let anno_host_port = 2300_u16;
        #[allow(unused)]
        let anno_host_addr_str = "10.30.0.2:2300";

        let tcp_listener = TcpListener::bind(listen_addr).await?;
        info!("[TCP] Listening on {:?}", listen_addr);

        // FIXME: handling connections based on host connecting to TcpProxy
        // currently wrongly named and connection handling should be moved
        // completely to corresponding tasks

        // Here we listen for an active inbound connection to the proxy.
        // Next we need to forward requests from the proxy to the target.
        // Anno 1602 has a quirk where we need a socket per direction,
        // meaning we have to handle two directions separately per client.

        while let Ok((stream, socket_addr)) = tcp_listener.accept().await {
            let tx_accumulator = tx_accumulator.clone();
            tokio::spawn(async move {
                let inbound_ip = socket_addr.ip();
                let inbound_port = socket_addr.port();

                let is_connection_from_anno_host = inbound_ip == anno_host_ip;
                info!(
                    "[TCP] Received connection from {:?}:{:?} | host={}",
                    inbound_ip, inbound_port, is_connection_from_anno_host
                );

                let destination_ip = if is_connection_from_anno_host {
                    IpAddr::V4(Ipv4Addr::new(10, 20, 0, 2))
                } else {
                    anno_host_ip
                };
                let destination_port = 2300_u16;
                let destination_addr_str = format!("{}:{}", destination_ip, destination_port);

                let (tx, mut rx) = mpsc::channel::<Message>(1);

                let mut stream_framed = BytesCodec::new().framed(stream);

                let handle_inbound = async move {
                    while let Some(message) = stream_framed.next().await {
                        match message {
                            Ok(bytes) => {
                                let data = &bytes[..];
                                trace!("Received {} bytes from client.", bytes.len());
                                let res = tx
                                    .send(Message {
                                        data: data.to_vec(),
                                        origin: (inbound_ip, inbound_port),
                                        destination: (destination_ip, destination_port),
                                        mask_as_address: None,
                                    })
                                    .await;
                                if res.is_err() {
                                    error!("{:?}", res);
                                }
                            }
                            Err(err) => info!("Socket closed with error: {:?}", err),
                        }
                    }
                    info!("Socket received FIN packet and closed connection");
                };
                let tx_accumulator = tx_accumulator.clone();
                let handle_outbound = async move {
                    let stream = TcpStream::connect(destination_addr_str).await.unwrap();
                    while let Some(mut message) = rx.recv().await {
                        let proxy_address = stream.local_addr().unwrap();
                        message.mask_as_address = Some((proxy_address.ip(), proxy_address.port()));
                        trace!("Proxy processing {} bytes.", message.data.len());
                        let (once_tx, once_rx) = oneshot::channel();
                        let _ = tx_accumulator.send((message, once_tx)).await;
                        let processed_message = once_rx.await.unwrap();
                        if let Ok(bytes_written) = stream.try_write(&processed_message.data) {
                            trace!("Sent {} bytes to host.", bytes_written);
                        }
                    }
                };

                let _ = tokio::join!(handle_inbound, handle_outbound);

                info!(
                    "Proxy thread done with connection from {}:{}",
                    inbound_ip, inbound_port
                );
            });
        }

        Ok(())
    }
}
