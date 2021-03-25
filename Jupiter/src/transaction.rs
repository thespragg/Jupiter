use std::fmt::{ Debug};

#[derive(Debug)]
pub struct Transaction {
    pub to_address : String,
    pub from_address : String,
    pub amnt : u64,
    pub hash : String
}