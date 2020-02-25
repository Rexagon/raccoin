use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub data: BlockData,

    pub previous_hash: Hash,
    pub hash: Hash,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockData {
    Text(String),
}

impl Block {
    pub fn new(index: u64, data: BlockData, previous_hash: &Hash) -> Self {
        let timestamp = Utc::now();
        let data = data.clone();
        let previous_hash: Hash = previous_hash.clone();
        let hash = Hash::new(&index, &previous_hash, &timestamp, &data);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn from_previous(previous: &Block, data: BlockData) -> Self {
        Block::new(previous.index + 1, data, &previous.hash)
    }

    pub fn validate_previous(&self, previous: &Block) -> bool {
        self.index == previous.index + 1
            && self.previous_hash == previous.hash
            && self.hash == Hash::from_block(self)
    }
}

impl Hash {
    pub fn from_block(block: &Block) -> Self {
        Hash::new(
            &block.index,
            &block.previous_hash,
            &block.timestamp,
            &block.data,
        )
    }
}
