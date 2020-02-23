use futures::{
    future::{self, Ready},
    prelude::*,
};
use std::{
    iter::FromIterator,
    sync::{Arc, Mutex},
};
use tarpc::context;

use crate::block::{Block, BlockData};
use crate::blockchain::BlockChain;
use crate::rpc::BlockChainService;

#[derive(Clone)]
pub struct Server(pub Arc<Mutex<BlockChain<BlockData>>>);

impl BlockChainService for Server {
    type SendLatestBlocksFut = Ready<bool>;

    fn send_latest_blocks(
        self,
        _: context::Context,
        blocks: Vec<Block<BlockData>>,
    ) -> Self::SendLatestBlocksFut {
        unimplemented!();
    }

    type FetchLatestBlocksFut = Ready<Vec<Block<BlockData>>>;

    fn fetch_latest_blocks(self, _: context::Context, n: usize) -> Self::FetchLatestBlocksFut {
        let blockchain = self.0.lock().unwrap();

        let result = Vec::from_iter(blockchain.blocks.iter().rev().take(n).rev().cloned());

        future::ready(result)
    }
}
