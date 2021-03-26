mod block;
pub use crate::block::Block;
mod transaction;
pub use crate::transaction::Transaction;
mod hash;
pub use crate::hash::ComputeHash;
use std::time::{SystemTime, UNIX_EPOCH};

type Hash = String;

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
    ;
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}