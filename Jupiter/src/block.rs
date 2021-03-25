use super::*;
use std::fmt::{ Debug };

#[derive(Debug)] 
pub struct Block {
    pub index : u32,
    pub timestamp : u128,
    pub transactions : Vec<Transaction>,
    pub hash : Vec<u8>,
    pub prev_block_hash : Vec<u8>,
    pub validator : String
}