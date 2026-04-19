use sha2::{Digest, Sha256};

pub fn hash_data(data: &str) -> String {
    let hash256 = Sha256::digest(data.as_bytes());
    hex::encode(hash256)
}
