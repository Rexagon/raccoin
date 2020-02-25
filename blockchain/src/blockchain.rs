use serde::Serialize;
use std::iter::FromIterator;

use crate::block::Block;

#[derive(Debug, Serialize)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new(genesis: Block) -> Self {
        BlockChain {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if !block.validate_previous(self.blocks.last().unwrap()) {
            return false;
        }

        self.blocks.push(block);
        true
    }

    pub fn tail(&self, n: usize) -> Vec<Block> {
        Vec::from_iter(self.blocks.iter().rev().take(n).rev().cloned())
    }

    pub fn last(&self) -> &Block {
        self.blocks.last().unwrap()
    }
}
