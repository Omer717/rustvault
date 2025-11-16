use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub version: u8,
    pub salt: String,
    pub check: EncryptedData,
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedData {
    pub nonce: String,      // base64
    pub ciphertext: String, // base64
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub service: String,
    pub username: Option<String>,
    pub nonce: String,      // AEAD nonce
    pub ciphertext: String, // encrypted password
}
