use std::env;
use std::error::Error;

use futures::FutureExt;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::try_join;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

pub struct TcpProxy {}

impl TcpProxy {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let listen_addr = "0.0.0.0:2300";
        let anno_host_addr = "10.30.0.2:2300";

        let tcp_listener = TcpListener::bind(listen_addr).await?;

        info!("[TCP] Listening on {:?}", listen_addr);

        while let Ok((mut inbound, socket_addr)) = tcp_listener.accept().await {
            info!("[TCP] Received connection from {:?}", socket_addr);

            tokio::spawn(async move {
                let mut framed = BytesCodec::new().framed(inbound);

                while let Some(message) = framed.next().await {
                    match message {
                        Ok(bytes) => {
                            let data = &bytes[..];
                            debug!("Incoming data:\n{:?}\n{:x?}", data, data)
                        }
                        Err(err) => info!("Socket closed with error: {:?}", err),
                    }
                }
                info!("Socket received FIN packet and closed connection");
            });

            // let mut outbound = TcpStream::connect(anno_host_addr).await?;

            // tokio::spawn(async move {
            //     copy_bidirectional(&mut inbound, &mut outbound)
            //         .map(|r| {
            //             if let Err(e) = r {
            //                 println!("Failed to transfer; error={}", e);
            //             }
            //         })
            //         .await
            // });
        }

        Ok(())
    }
}
