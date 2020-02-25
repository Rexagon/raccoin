pub mod server;

pub use server::*;

use futures::{future, prelude::*};
use std::net::SocketAddr;
use tarpc::{client, server::Channel};
use tokio_serde::formats::Bincode;

use crate::block::Block;
use crate::state::State;

#[tarpc::service]
pub trait BlockChainService {
    async fn send_latest_blocks(blocks: Vec<Block>) -> bool;

    async fn fetch_latest_blocks(n: usize) -> Vec<Block>;
}

pub async fn serve(addr: SocketAddr, shared_state: State) -> std::io::Result<()> {
    tarpc::serde_transport::tcp::listen(&addr, Bincode::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = Server(shared_state.clone());
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}

pub async fn connect(addr: SocketAddr) -> std::io::Result<BlockChainServiceClient> {
    let other_server = tarpc::serde_transport::tcp::connect(&addr, Bincode::default()).await?;
    let client = BlockChainServiceClient::new(client::Config::default(), other_server).spawn()?;

    Ok(client)
}
