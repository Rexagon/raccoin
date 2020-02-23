use clap::{App, Arg};
use std::net::SocketAddr;
use tarpc::{client, context};
use tokio_serde::formats::Bincode;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let flags = App::new("test-service-client")
        .version("0.1")
        .arg(
            Arg::with_name("server_addr")
                .long("server_addr")
                .value_name("ADDRESS")
                .help("Sets the server address to connect to.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("STRING")
                .help("Sets the name to say hello to")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let server_addr = flags.value_of("server_addr").unwrap();
    let server_addr = server_addr
        .parse::<SocketAddr>()
        .unwrap_or_else(|e| panic!(r#"--server_addr value "{}" invalid: {}"#, server_addr, e));

    let name = flags.value_of("name").unwrap().into();

    let transport = tarpc::serde_transport::tcp::connect(server_addr, Bincode::default()).await?;

    let mut client =
        service::WorldServiceClient::new(client::Config::default(), transport).spawn()?;

    let hello = client.hello(context::current(), name).await?;
    println!("{}", hello);

    Ok(())
}
