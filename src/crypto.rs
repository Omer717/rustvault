use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KdfParams {
    pub algorithm: String, // "argon2id"
    pub salt: Vec<u8>,     // random salt
    pub iterations: u32,
    pub memory_mb: u32,
    pub threads: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    pub algorithm: String, // "aes-gcm"
    pub nonce: Vec<u8>,    // random nonce used for encryption
}
