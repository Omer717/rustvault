use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: u32,          // unique identifier
    pub service: String,  // e.g., "Gmail", "GitHub"
    pub username: String, // username/email for the service
    pub password: String, // the password itself (encrypted in storage)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<Entry>, // all password entries
    pub salt: Vec<u8>,       // salt used to derive encryption key from master password
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub version: u8,
    pub kdf: KdfParams,
    pub encryption: EncryptionMetadata,
    pub ciphertext: Vec<u8>, // base64 in JSON
}
