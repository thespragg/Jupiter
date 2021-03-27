use std::fmt::{ Debug};
use super::*;

#[derive(Debug)]
pub struct Transaction {
    pub to_address : Hash,
    pub from_address : Hash,
    pub amnt : u64,
    pub hash : Hash
}

impl Hashable for Transaction {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&*self.to_address.as_bytes());
        bytes.extend(&*self.from_address.as_bytes());
        bytes.extend(&self.amnt.to_be_bytes());
        bytes.extend(&*self.hash.as_bytes());
        bytes
    }
}