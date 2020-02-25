use std::sync::{Arc, Mutex};

use crate::blockchain::BlockChain;
use crate::peers_network::PeersNetwork;
use crate::settings::Settings;

#[derive(Clone)]
pub struct State {
    pub blockchain: Arc<Mutex<BlockChain>>,
    pub peers: Arc<Mutex<PeersNetwork>>,
}

impl State {
    pub fn new(settings: &Settings) -> Self {
        State {
            blockchain: Arc::new(Mutex::new(BlockChain::new(settings.genesis.clone()))),
            peers: Arc::new(Mutex::new(PeersNetwork::new())),
        }
    }
}
