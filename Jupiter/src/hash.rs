pub trait ComputeHash {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> String {
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}