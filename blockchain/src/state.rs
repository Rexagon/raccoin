use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::blockchain::BlockChain;
use crate::rpc;
use crate::settings::Settings;

#[derive(Clone)]
pub struct State {
    pub blockchain: Arc<Mutex<BlockChain>>,
    pub peers: Arc<Mutex<Vec<Peer>>>,
}

pub type Peer = (SocketAddr, rpc::BlockChainServiceClient);

impl State {
    pub fn new(settings: &Settings) -> Self {
        State {
            blockchain: Arc::new(Mutex::new(BlockChain::new(settings.genesis.clone()))),
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
