use serde::{Deserialize, Serialize};

use crate::block::Block;

#[derive(Debug, Serialize)]
pub struct BlockChain<T> {
    pub blocks: Vec<Block<T>>,
}

impl<'de, T> BlockChain<T>
where
    T: Serialize + Deserialize<'de> + Clone,
{
    pub fn new(genesis: Block<T>) -> Self {
        BlockChain {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, block: Block<T>) -> bool {
        if !block.validate_previous(self.blocks.last().unwrap()) {
            return false;
        }

        self.blocks.push(block);
        true
    }

    pub fn last(&self) -> &Block<T> {
        self.blocks.last().unwrap()
    }
}
