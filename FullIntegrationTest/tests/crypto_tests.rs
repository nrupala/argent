use argent::{ArgentVault, VaultError};
use std::path::PathBuf;
use tempfile::TempDir;

fn get_test_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[tokio::test]
async fn test_vault_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    
    let result = vault.initialize(Some(b"01234567890123456789012345678901")).await;
    assert!(result.is_ok(), "Vault initialization should succeed");
    
    println!("TEST PASSED: vault_initialization");
    Ok(())
}

#[tokio::test]
async fn test_vault_encrypt_decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    
    vault.initialize(Some(b"01234567890123456789012345678901")).await?;
    
    let plaintext = "Hello, Argent!";
    let encrypted = vault.encrypt(plaintext).await?;
    let decrypted = vault.decrypt(&encrypted).await?;
    
    assert_eq!(plaintext, decrypted, "Decrypted text should match original");
    
    println!("TEST PASSED: vault_encrypt_decrypt");
    Ok(())
}

#[tokio::test]
async fn test_vault_state_persistence() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    
    vault.initialize(None).await?;
    
    let state_data = r#"{"key": "value", "count": 42}"#;
    let store_result = vault.store_state("test_state", state_data).await;
    assert!(store_result.is_ok(), "State store should succeed");
    
    let load_result = vault.load_state("test_state").await;
    assert!(load_result.is_ok(), "State load should succeed");
    
    println!("TEST PASSED: vault_state_persistence");
    Ok(())
}

#[tokio::test]
async fn test_vault_is_initialized() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    
    let before = vault.is_initialized().await;
    assert!(!before, "Vault should not be initialized initially");
    
    vault.initialize(None).await?;
    
    let after = vault.is_initialized().await;
    assert!(after, "Vault should be initialized after init");
    
    println!("TEST PASSED: vault_is_initialized");
    Ok(())
}

#[tokio::test]
async fn test_vault_storage_path() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    
    vault.initialize(None).await?;
    
    let path = vault.storage_path().await;
    assert!(path.exists(), "Storage path should exist");
    
    println!("TEST PASSED: vault_storage_path");
    Ok(())
}