use super::*;
use std::fmt::{ Debug };

#[derive(Debug)] 
pub struct Block {
    pub index : u32,
    pub timestamp : u128,
    pub transactions : Vec<Transaction>,
    pub hash : Hash,
    pub prev_block_hash : Hash,
    pub validator : Hash
}

impl Block {
    pub fn generate_genesis() -> Block{
        let mut block = Block {
            index: 0,
            timestamp: now(),
            transactions: Vec::new(),
            hash: String::new(),
            prev_block_hash: String::new(),
            validator: String::new()
        };
        block.hash = block.hash();
        block
    }

    pub fn next (old_block: &Block, validator: Hash) -> Block {
        let mut block = Block {
            index: old_block.index + 1,
            timestamp: now(),
            transactions: Vec::new(),
            hash: String::new(),
            prev_block_hash: old_block.hash.clone(),
            validator: validator,
        };
        block.hash = block.hash();
        block
    }
}

impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        bytes.extend(&*self.prev_block_hash.as_bytes());
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}