//! Encrypted Secret Vault
//!
//! Provides a layered security approach to storing sensitive data.
//! Secrets are encrypted with AES-256-GCM using hardware-backed keys.

use crate::{FluxResult, FluxError};
use crate::security::keychain::KeychainManager;
use std::collections::HashMap;

pub struct EncryptedVault {
    keychain: KeychainManager,
    // Key: Identifier, Value: Base64 Encoded AES-256-GCM Ciphertext
    encrypted_data: HashMap<String, String>,
}

impl EncryptedVault {
    pub fn new(biometric_token: &str) -> FluxResult<Self> {
        Ok(Self {
            keychain: KeychainManager::new(biometric_token)?,
            encrypted_data: HashMap::new(),
        })
    }

    pub fn store(&mut self, key: &str, value: &str) -> FluxResult<()> {
        let ciphertext = self.keychain.encrypt_secret(value)?;
        self.encrypted_data.insert(key.into(), ciphertext);
        Ok(())
    }

    pub fn retrieve(&self, key: &str) -> FluxResult<String> {
        let ciphertext = self.encrypted_data.get(key)
            .ok_or_else(|| FluxError::Security(format!("Secret '{}' not found", key)))?;
        
        self.keychain.decrypt_secret(ciphertext)
    }

    pub fn delete(&mut self, key: &str) {
        self.encrypted_data.remove(key);
    }
}
