use futures::Future;
use std::net::SocketAddr;

use crate::rpc;

pub struct PeersNetwork(Vec<Peer>);

pub struct Peer(SocketAddr, rpc::BlockChainServiceClient);

impl Peer {
    pub async fn try_create(addr: SocketAddr) -> std::io::Result<Self> {
        let peer = rpc::connect(addr).await?;

        Ok(Peer(addr, peer))
    }
}

impl PeersNetwork {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn peers(&self) -> Vec<SocketAddr> {
        self.0.iter().map(|Peer(addr, _)| *addr).collect()
    }

    pub fn add(&mut self, peer: Peer) {
        self.0.push(peer);
    }

    pub fn remove(&mut self, addr: SocketAddr) {
        let index = self
            .0
            .iter()
            .position(|Peer(peer_addr, _)| *peer_addr == addr);
        if let Some(index) = index {
            self.0.remove(index);
        }
    }

    pub async fn for_each<P, F>(&mut self, predicate: P)
    where
        P: FnMut(&mut Peer) -> F,
        F: Future<Output = ()>,
    {
        futures::future::join_all(self.0.iter_mut().map(predicate)).await;
    }
}
