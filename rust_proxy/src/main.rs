#![warn(rust_2018_idioms)]

mod tcp_proxy;
mod udp_proxy;

#[hot_lib_reloader::hot_module(dylib = "x642")]
mod x642 {
    hot_functions_from_file!("x642/src/lib.rs");

    pub use proxy_commons::Message;
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

use std::error::Error;

pub use proxy_commons::Message;
use tokio::sync::{mpsc, oneshot};
use tokio::try_join;
use tracing::{trace, error, Level, span};

use crate::tcp_proxy::TcpProxy;
use crate::udp_proxy::UdpProxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lib_observer = x642::subscribe();

    {
        // Setting up color_eyre and tracing for debug output

        let _ = color_eyre::install();

        let mut subscriber = tracing_subscriber::fmt();
        if !cfg!(debug_assertions) {
            subscriber = subscriber.with_max_level(Level::INFO);
        } else {
            subscriber = subscriber.with_max_level(Level::DEBUG);
        }

        subscriber.init();
    }

    tokio::spawn(async move {
        loop {
            x642::set_shared_logger(proxy_commons::shared_logger::build_shared_logger());
            lib_observer.wait_for_reload();
        }
    });

    let (tx, mut rx) = 
        // mpsc - multiple producer, single consumer + oneshot to send messages back to origin
        mpsc::channel::<(Message, oneshot::Sender<Message>)>(32);

    tokio::spawn(async move {
        let accumulator_span = span!(Level::DEBUG, "Accumulator");
        while let Some((message, responder)) = rx.recv().await {
            let mut response = message;
            
            let entered = accumulator_span.enter();
            trace!("Received: {:?}", response);
            drop(entered);

            x642::parse_message(&mut response);

            trace!("Sending: {:?}", response);

            let _ = responder.send(response);
        }
    });

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
