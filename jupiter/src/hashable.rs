use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> String{
        let mut hasher = Sha3::keccak256();
        hasher.input(&self.bytes());
        let hex = hasher.result_str();
        hex
    }
}
