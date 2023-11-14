#![warn(rust_2018_idioms)]
#![allow(unused)]

mod tcp_proxy;
mod udp_proxy;

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

use crate::tcp_proxy::TcpProxy;
use crate::udp_proxy::UdpProxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = color_eyre::install();

    let mut subscriber = FmtSubscriber::builder();
    if !cfg!(debug_assertions) {
        subscriber = subscriber.with_max_level(Level::INFO);
    } else {
        subscriber = subscriber.with_max_level(Level::DEBUG);
    }

    tracing::subscriber::set_global_default(subscriber.finish())
        .expect("setting default subscriber failed");

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:47624".to_string());

    let udp_proxy = UdpProxy::new(addr, 1024).await?;
    let tcp_proxy = TcpProxy::new().await?;

    let res = try_join!(udp_proxy.run(), tcp_proxy.run());

    if res.is_err() {
        let err = res.unwrap_err();
        error!("{:?}", err);
        Err(err)
    } else {
        Ok(())
    }
}
