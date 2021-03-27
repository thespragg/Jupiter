use super::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub candidates: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut chain = Blockchain {
            blocks: vec![],
            candidates: vec![],
            pending_transactions: vec![],
        };
        chain.blocks.push(Block::generate_genesis());
        chain
    }
}

impl Blockchain {
    pub fn get_latest_block(&self) -> &Block {
        let block = &self.blocks[&self.blocks.len() - 1];
        block
    }

    pub fn get_previous_block(&self) -> &Block {
        let block = &self.blocks[&self.blocks.len() - 2];
        block
    }

    pub fn insert_block(&mut self, block: Block) {
        &self.blocks.push(block);
    }

    pub fn is_valid(&self) -> bool {
        let current_block = &self.get_latest_block();
        let previous_block = &self.get_previous_block();
        if current_block.hash != current_block.hash() {
            false;
        }
        if current_block.prev_block_hash != previous_block.hash() {
            false;
        }
        true
    }
}
