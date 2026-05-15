pub mod keychain;
pub mod firewall;
pub mod vault;

use crate::{FluxResult, FluxError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRule {
    pub pattern: String,
    pub action: SafetyAction,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SafetyAction { Allow, Warn, Block }

pub struct SecurityManager {
    pub encrypted_vault: Option<vault::EncryptedVault>,
    pub firewall: firewall::CommandFirewall,
    pub biometric_enabled: bool,
    pub is_locked: bool,
    data_dir: String,
}

impl SecurityManager {
    pub fn new(data_dir: &str) -> FluxResult<Self> {
        tracing::info!("Initializing SecurityManager with layered protection in {}", data_dir);
        Ok(Self {
            encrypted_vault: None, // Vault remains null until biometric unlock
            firewall: firewall::CommandFirewall::new(),
            biometric_enabled: false,
            is_locked: true,
            data_dir: data_dir.into(),
        })
    }

    /// Primary Security Entry Point: Unlock the encrypted layers using biometric hardware token
    pub fn unlock_hardware_layer(&mut self, biometric_token: &str) -> FluxResult<()> {
        tracing::info!("Layer 1: Decrypting Hardware Vault...");
        self.encrypted_vault = Some(vault::EncryptedVault::new(biometric_token)?);
        self.is_locked = false;
        Ok(())
    }

    pub fn check_command_safety(&self, command: &str) -> SafetyAction {
        self.firewall.analyze(command)
    }

    pub fn store_secret(&mut self, key: &str, value: &str) -> FluxResult<()> {
        if let Some(v) = &mut self.encrypted_vault {
            v.store(key, value)
        } else {
            Err(FluxError::Security("Vault is locked".into()))
        }
    }

    pub fn get_secret(&self, key: &str) -> FluxResult<String> {
        if let Some(v) = &self.encrypted_vault {
            v.retrieve(key)
        } else {
            Err(FluxError::Security("Vault is locked".into()))
        }
    }

    pub fn lock(&mut self) {
        self.encrypted_vault = None;
        self.is_locked = true;
    }

    pub fn data_dir(&self) -> &str { &self.data_dir }
}
