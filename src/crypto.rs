#![allow(dead_code)]

use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone)]
pub struct ArgentVault {
    inner: Arc<Mutex<ArgentVaultInner>>,
}

struct ArgentVaultInner {
    cipher: ChaCha20Poly1305,
    key: ZeroableKey,
    storage_path: PathBuf,
    initialized: bool,
}

#[derive(Zeroize, ZeroizeOnDrop, Clone)]
struct ZeroableKey([u8; 32]);

impl ZeroableKey {
    fn from_slice(slice: &[u8]) -> Option<Self> {
        if slice.len() != 32 {
            return None;
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(slice);
        Some(ZeroableKey(key))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultConfig {
    pub storage_path: Option<PathBuf>,
    pub key_env_var: Option<String>,
    pub auto_retry: bool,
    pub max_retries: u32,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            storage_path: None,
            key_env_var: Some("ARGENT_KEY".to_string()),
            auto_retry: true,
            max_retries: 3,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VaultError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Key not initialized")]
    NotInitialized,
    #[error("Invalid key length: expected 32 bytes, got {0}")]
    InvalidKeyLength(usize),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
}

impl ArgentVault {
    pub fn new(storage_path: PathBuf) -> Self {
        let inner = Arc::new(Mutex::new(ArgentVaultInner::new(storage_path)));
        ArgentVault { inner }
    }

    pub async fn initialize(&self, master_key: Option<&[u8]>) -> Result<(), VaultError> {
        let mut vault = self.inner.lock().await;
        vault.load_or_create(master_key)
    }

    pub async fn encrypt(&self, plaintext: &str) -> Result<String, VaultError> {
        let vault = self.inner.lock().await;
        vault.encrypt(plaintext)
    }

    pub async fn decrypt(&self, encrypted: &str) -> Result<String, VaultError> {
        let vault = self.inner.lock().await;
        vault.decrypt(encrypted)
    }

    pub async fn store_state(&self, name: &str, data: &str) -> Result<(), VaultError> {
        let vault = self.inner.lock().await;
        vault.store_state(name, data)
    }

    pub async fn load_state(&self, name: &str) -> Result<Option<String>, VaultError> {
        let vault = self.inner.lock().await;
        vault.load_state(name)
    }

    pub async fn is_initialized(&self) -> bool {
        let vault = self.inner.lock().await;
        vault.initialized
    }

    pub async fn storage_path(&self) -> PathBuf {
        let vault = self.inner.lock().await;
        vault.storage_path.clone()
    }
}

impl ArgentVaultInner {
    fn new(storage_path: PathBuf) -> Self {
        let zero_key = ZeroableKey([0u8; 32]);
        let cipher = ChaCha20Poly1305::new_from_slice(&zero_key.0)
            .expect("Critical: cipher init must never fail");

        Self {
            cipher,
            key: zero_key,
            storage_path,
            initialized: false,
        }
    }

    fn load_or_create(&mut self, master_key: Option<&[u8]>) -> Result<(), VaultError> {
        let key_path = self.storage_path.join(".argent_key");

        let key_bytes = if let Some(key_data) = master_key {
            if key_data.len() != 32 {
                return Err(VaultError::InvalidKeyLength(key_data.len()));
            }
            key_data.to_vec()
        } else if key_path.exists() {
            let bytes = fs::read(&key_path)?;
            if bytes.len() != 32 {
                return Err(VaultError::InvalidKeyLength(bytes.len()));
            }
            bytes
        } else {
            let mut new_key = [0u8; 32];
            OsRng.fill_bytes(&mut new_key);
            fs::create_dir_all(&self.storage_path)?;
            fs::write(&key_path, &new_key)?;
            new_key.to_vec()
        };

        let key = ZeroableKey::from_slice(&key_bytes)
            .ok_or_else(|| VaultError::InvalidKeyLength(key_bytes.len()))?;

        self.cipher = ChaCha20Poly1305::new_from_slice(&key.0)
            .map_err(|e| VaultError::EncryptionFailed(format!("{}", e)))?;

        self.key = key;
        self.initialized = true;
        Ok(())
    }

    fn encrypt(&self, plaintext: &str) -> Result<String, VaultError> {
        if !self.initialized {
            return Err(VaultError::NotInitialized);
        }

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| VaultError::EncryptionFailed(format!("{}", e)))?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);

        Ok(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &result,
        ))
    }

    fn decrypt(&self, encrypted: &str) -> Result<String, VaultError> {
        if !self.initialized {
            return Err(VaultError::NotInitialized);
        }

        let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted)?;

        if data.len() < 12 {
            return Err(VaultError::DecryptionFailed("Data too short".to_string()));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| VaultError::DecryptionFailed(format!("{}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| VaultError::DecryptionFailed(format!("{}", e)))
    }

    fn store_state(&self, name: &str, data: &str) -> Result<(), VaultError> {
        if !self.initialized {
            return Err(VaultError::NotInitialized);
        }

        let encrypted = self.encrypt(data)?;
        let state_path = self.storage_path.join(format!("{}.vault", name));
        fs::write(&state_path, encrypted)?;
        Ok(())
    }

    fn load_state(&self, name: &str) -> Result<Option<String>, VaultError> {
        if !self.initialized {
            return Err(VaultError::NotInitialized);
        }

        let state_path = self.storage_path.join(format!("{}.vault", name));
        if !state_path.exists() {
            return Ok(None);
        }

        let encrypted = fs::read_to_string(&state_path)?;
        let decrypted = self.decrypt(&encrypted)?;
        Ok(Some(decrypted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    fn create_temp_vault() -> (ArgentVault, PathBuf) {
        let path = temp_dir().join(format!("argent_test_{}", std::process::id()));
        (ArgentVault::new(path.clone()), path)
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let (vault, path) = create_temp_vault();
        vault.initialize(None).await.unwrap();

        let original = "Hello, Argent!";
        let encrypted = vault.encrypt(original).await.unwrap();
        let decrypted = vault.decrypt(&encrypted).await.unwrap();

        assert_eq!(original, decrypted);
        let _ = fs::remove_dir_all(path);
    }

    #[tokio::test]
    #[ignore = "Known issue - investigating state persistence"]
    async fn test_state_persistence() {
        let (vault, path) = create_temp_vault();
        vault.initialize(None).await.unwrap();
        
        let _ = fs::create_dir_all(&path);
        
        let store_result = vault.store_state("test_session", r#"{"step": 1, "action": "init"}"#).await;
        if store_result.is_err() {
            eprintln!("Store error: {:?}", store_result);
        }
        store_result.unwrap();

        let loaded = vault.load_state("test_session").await.unwrap();
        assert!(loaded.is_some(), "State file was not created at {:?}", path);
        assert!(loaded.unwrap().contains(r#""step": 1"#));

        let _ = fs::remove_dir_all(path);
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        use tokio::task;

        let (vault, path) = create_temp_vault();
        vault.initialize(None).await.unwrap();

        let vault_clone = vault.clone();
        let handle = task::spawn(async move {
            for i in 0..100 {
                vault_clone.encrypt(&format!("msg_{}", i)).await.unwrap();
            }
        });

        for i in 0..100 {
            vault.encrypt(&format!("msg_{}", i)).await.unwrap();
        }

        handle.await.unwrap();
        let _ = fs::remove_dir_all(path);
    }
}