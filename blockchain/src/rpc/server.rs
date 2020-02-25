use futures::future::{self, Ready};
use tarpc::context;

use crate::block::Block;
use crate::rpc::BlockChainService;
use crate::state::State;

#[derive(Clone)]
pub struct Server(pub State);

impl BlockChainService for Server {
    type SendLatestBlocksFut = Ready<bool>;

    fn send_latest_blocks(
        self,
        _: context::Context,
        _blocks: Vec<Block>,
    ) -> Self::SendLatestBlocksFut {
        unimplemented!();
    }

    type FetchLatestBlocksFut = Ready<Vec<Block>>;

    fn fetch_latest_blocks(self, _: context::Context, n: usize) -> Self::FetchLatestBlocksFut {
        let blockchain = self.0.blockchain.lock().unwrap();

        future::ready(blockchain.tail(n))
    }
}
