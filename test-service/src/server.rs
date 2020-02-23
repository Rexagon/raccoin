use clap::{App, Arg};
use futures::{
    future::{self, Ready},
    prelude::*,
};
use std::net::{IpAddr, SocketAddr};
use tarpc::{
    context,
    server::{self, Channel, Handler},
};
use tokio_serde::formats::Bincode;

use service::WorldService;

#[derive(Clone)]
pub struct MyServer(SocketAddr);

impl WorldService for MyServer {
    type HelloFut = Ready<String>;

    fn hello(self, _: context::Context, name: String) -> Self::HelloFut {
        future::ready(format!("[{:?}]: Hello, {}!", self.0, name))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let flags = App::new("test-service")
        .version("0.1")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("NUMBER")
                .help("Sets the port number to listen on")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let port = flags.value_of("port").unwrap();
    let port = port
        .parse()
        .unwrap_or_else(|e| panic!(r#"--port value "{}" invalid: {}"#, port, e));

    let server_addr = (IpAddr::from([0, 0, 0, 0]), port);

    tarpc::serde_transport::tcp::listen(&server_addr, Bincode::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = MyServer(channel.as_ref().as_ref().peer_addr().unwrap());
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
