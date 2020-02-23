pub mod server;

pub use server::*;

use futures::{future, prelude::*};
use std::sync::{Arc, Mutex};
use tarpc::server::Channel;
use tokio::net::ToSocketAddrs;
use tokio_serde::formats::Bincode;

use crate::block::{Block, BlockData};
use crate::blockchain::BlockChain;

#[tarpc::service]
pub trait BlockChainService {
    async fn send_latest_blocks(blocks: Vec<Block<BlockData>>) -> bool;

    async fn fetch_latest_blocks(n: usize) -> Vec<Block<BlockData>>;
}

pub async fn serve<T>(addr: T, data: Arc<Mutex<BlockChain<BlockData>>>) -> std::io::Result<()>
where
    T: ToSocketAddrs,
{
    tarpc::serde_transport::tcp::listen(&addr, Bincode::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = Server(data.clone());
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
