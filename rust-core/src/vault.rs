use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use anyhow::{Context, Result};
use rand::RngCore;
use std::fs;
use std::path::Path;

pub struct Vault {
    cipher: Aes256Gcm,
}

impl Vault {
    pub fn new(password: &str) -> Result<Self> {
        // In a production system, use PBKDF2 or Argon2 for key derivation.
        // For this implementation, we'll hash the password to 32 bytes (256 bits).
        let key_bytes = blake3::hash(password.as_bytes());
        let key = Key::<Aes256Gcm>::from_slice(key_bytes.as_bytes());
        let cipher = Aes256Gcm::new(key);
        Ok(Vault { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = self.cipher.encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failure: {}", e))?;
        
        // Return nonce + ciphertext as hex string
        let mut combined = nonce.to_vec();
        combined.extend(ciphertext);
        Ok(hex::encode(combined))
    }

    pub fn decrypt(&self, encrypted_hex: &str) -> Result<String> {
        let encrypted_bytes = hex::decode(encrypted_hex).context("Invalid hex string")?;
        
        if encrypted_bytes.len() < 12 {
            return Err(anyhow::anyhow!("Ciphertext too short"));
        }

        let (nonce_bytes, ciphertext_bytes) = encrypted_bytes.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext_bytes = self.cipher.decrypt(nonce, ciphertext_bytes)
            .map_err(|e| anyhow::anyhow!("Decryption failure: {}", e))?;
            
        String::from_utf8(plaintext_bytes).context("Invalid UTF-8")
    }
}
