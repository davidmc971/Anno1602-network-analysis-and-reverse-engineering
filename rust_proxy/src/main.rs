#![warn(rust_2018_idioms)]

mod udp_proxy;

use std::env;
use std::error::Error;

use futures::FutureExt;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::try_join;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::udp_proxy::UdpServer;

async fn tcp_server() -> Result<(), Box<dyn Error>> {
    let listen_addr = "0.0.0.0:2300";
    let anno_host_addr = "10.30.0.2:2300";

    let tcp_listener = TcpListener::bind(listen_addr).await?;

    info!("[TCP] Listening on {:?}", listen_addr);

    while let Ok((mut inbound, socket_addr)) = tcp_listener.accept().await {
        info!("[TCP] Received connection from {:?}", socket_addr);
        let mut outbound = TcpStream::connect(anno_host_addr).await?;

        tokio::spawn(async move {
            copy_bidirectional(&mut inbound, &mut outbound)
                .map(|r| {
                    if let Err(e) = r {
                        println!("Failed to transfer; error={}", e);
                    }
                })
                .await
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = color_eyre::install();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:47624".to_string());

    let server = UdpServer::new(addr, 1024).await?;

    let res = try_join!(server.run(), tcp_server());

    if res.is_err() {
        let err = res.unwrap_err();
        error!("{:?}", err);
        Err(err)
    } else {
        Ok(())
    }
}
