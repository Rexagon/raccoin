use futures::future::{self, Ready};
use std::iter::FromIterator;
use tarpc::context;

use crate::block::{Block, BlockData};
use crate::rpc::BlockChainService;
use crate::SharedState;

#[derive(Clone)]
pub struct Server(pub SharedState<BlockData>);

impl BlockChainService for Server {
    type SendLatestBlocksFut = Ready<bool>;

    fn send_latest_blocks(
        self,
        _: context::Context,
        _blocks: Vec<Block<BlockData>>,
    ) -> Self::SendLatestBlocksFut {
        unimplemented!();
    }

    type FetchLatestBlocksFut = Ready<Vec<Block<BlockData>>>;

    fn fetch_latest_blocks(self, _: context::Context, n: usize) -> Self::FetchLatestBlocksFut {
        let blockchain = self.0.blockchain.lock().unwrap();

        let result = Vec::from_iter(blockchain.blocks.iter().rev().take(n).rev().cloned());

        future::ready(result)
    }
}
