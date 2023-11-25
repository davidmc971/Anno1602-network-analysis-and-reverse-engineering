#![warn(rust_2018_idioms)]
#![allow(unused)]

mod parser;
mod tcp_proxy;
mod udp_proxy;

use std::env;
use std::error::Error;

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
        mpsc::channel::<(String, oneshot::Sender<String>)>(32);

    tokio::spawn(async move {
        let accumulator_span = span!(Level::DEBUG, "Accumulator");
        while let Some((message, responder)) = rx.recv().await {
            let response = message.clone();

            {
                let entered = accumulator_span.enter();
                debug!("Received: {}", response);
                drop(entered);
            }

            let _ = responder.send(response);
        }
    });

    for i in 0..2 {
        let tx = tx.clone();
        let thread_span = span!(Level::DEBUG, "Thread", num = i);
        tokio::spawn(async move {
            for j in 0..10 {
                let (res_tx, res_rx) = oneshot::channel();
                let _ = tx.send((format!("Message from thread {}: {}", i, j), res_tx)).await;
                let x = res_rx.await;
                let entered = thread_span.enter();
                debug!("{}", x.unwrap());
                drop(entered);
            }
        });
    }
    

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
