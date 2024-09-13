pub mod walrus;
pub use walrus::{read_blob, store_blob};

pub fn calculate_sha256_digest(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
