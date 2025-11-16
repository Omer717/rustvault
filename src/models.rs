use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub version: u8,
    pub kdf: KdfConfig,
    pub check: EncryptedData,
    pub secrets: Vec<Secret>,
}

#[derive(Serialize, Deserialize)]
pub struct KdfConfig {
    pub salt: String, // base64
    pub memory_cost: u32,
    pub iterations: u32,
    pub parallelism: u8,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedData {
    pub nonce: String,      // base64
    pub ciphertext: String, // base64
}

#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub service: String,
    pub username: Option<String>,
    pub nonce: String,      // AEAD nonce
    pub ciphertext: String, // encrypted password
}

impl Zeroize for Secret {
    fn zeroize(&mut self) {
        self.ciphertext.zeroize();
    }
}
