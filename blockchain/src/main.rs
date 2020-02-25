mod block;
mod blockchain;
mod hash;
mod rest_api;
mod rpc;
mod settings;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::blockchain::BlockChain;
use crate::settings::Settings;

#[derive(Clone)]
pub struct SharedState<T> {
    pub blockchain: Arc<Mutex<BlockChain<T>>>,
    pub peers: Arc<Mutex<Vec<(SocketAddr, rpc::BlockChainServiceClient)>>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let settings = Settings::new().expect("Unable to parse settings.json");

    let shared_state = SharedState {
        blockchain: Arc::new(Mutex::new(BlockChain::new(settings.genesis))),
        peers: Arc::new(Mutex::new(Vec::new())),
    };

    tokio::spawn(rpc::serve(settings.api.rpc, shared_state.clone()));

    rest_api::serve(settings.api.rest, shared_state).await;

    Ok(())
}
