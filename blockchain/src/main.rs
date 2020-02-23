mod block;
mod blockchain;
mod hash;
mod rest_api;
mod rpc;

use std::{
    net::IpAddr,
    sync::{Arc, Mutex},
};

use crate::block::{Block, BlockData};
use crate::blockchain::BlockChain;
use crate::hash::Hash;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let genesis = Block::new(0, BlockData::Text("genesis".to_string()), &Hash::default());
    let blockchain = Arc::new(Mutex::new(BlockChain::new(genesis)));

    tokio::spawn(rpc::serve(
        (IpAddr::from([0, 0, 0, 0]), 11000),
        blockchain.clone(),
    ));

    rest_api::serve(([0, 0, 0, 0], 8080), blockchain).await;

    Ok(())
}
