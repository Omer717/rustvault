use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use argon2::{self};
use base64::{Engine, engine::general_purpose};
use rand::{TryRngCore, rngs::OsRng};

use crate::models::EncryptedData;

pub fn derive_key(master_password: &str, salt: &str) -> anyhow::Result<Vec<u8>> {
    let salt = general_purpose::STANDARD.decode(salt)?;
    let argon2 = argon2::Argon2::default();
    let mut key = vec![0u8; 32];
    argon2
        .hash_password_into(master_password.as_bytes(), &salt, &mut key)
        .unwrap();
    Ok(key)
}

pub fn get_salt() -> String {
    let mut salt = [0u8; 16]; // 16 bytes is a good standard size
    OsRng.try_fill_bytes(&mut salt).unwrap(); // Fill with cryptographically secure random bytes
    general_purpose::STANDARD.encode(&salt) // Encode to base64 for storage
}

pub fn encrypt_check(master_key: &[u8]) -> EncryptedData {
    encrypt(master_key, "vault check")
}

pub fn test_master_key(master_key: &[u8], encrypted_check: &EncryptedData) -> anyhow::Result<bool> {
    let Ok(plaintext) = decrypt(master_key, encrypted_check) else {
        return Ok(false); // decryption failed (wrong key or corrupted data)
    };

    Ok(plaintext == "vault check")
}

pub fn get_nonce() -> [u8; 12] {
    let mut nonce_bytes = [0u8; 12];
    OsRng.try_fill_bytes(&mut nonce_bytes).unwrap(); // Fill with cryptographically secure random bytes
    nonce_bytes
}

pub fn encrypt(master_key: &[u8], plaintext: &str) -> EncryptedData {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(master_key));

    let nonce_bytes = get_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).unwrap();

    EncryptedData {
        nonce: general_purpose::STANDARD.encode(nonce.as_slice()), // slice needed
        ciphertext: general_purpose::STANDARD.encode(&ciphertext),
    }
}

pub fn decrypt(master_key: &[u8], encrypted_data: &EncryptedData) -> anyhow::Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(master_key));

    let nonce_bytes = general_purpose::STANDARD.decode(&encrypted_data.nonce)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext_bytes = general_purpose::STANDARD.decode(&encrypted_data.ciphertext)?;

    let plaintext_bytes = cipher
        .decrypt(nonce, ciphertext_bytes.as_ref())
        .map_err(|e| anyhow::anyhow!("AES-GCM decryption failed: {:?}", e))?;
    let plaintext = String::from_utf8(plaintext_bytes)?;

    Ok(plaintext)
}
