use argent::sandbox::{ArgentSandbox, SandboxError};
use tempfile::TempDir;

fn get_test_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[tokio::test]
async fn test_sandbox_creation() {
    let sandbox = ArgentSandbox::new();
    assert!(!sandbox.is_initialized().await, "Sandbox should not be initialized initially");
    println!("TEST_PASSED: sandbox_creation");
}

#[tokio::test]
async fn test_sandbox_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let sandbox_dir = temp_dir.path().join("sandbox");
    std::fs::create_dir_all(&sandbox_dir)?;
    
    let sandbox = ArgentSandbox::new();
    let result = sandbox.init(sandbox_dir).await;
    assert!(result.is_ok(), "Sandbox init should succeed");
    
    let initialized = sandbox.is_initialized().await;
    assert!(initialized, "Sandbox should be initialized after init");
    
    println!("TEST_PASSED: sandbox_initialization");
    Ok(())
}

#[tokio::test]
async fn test_sandbox_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let sandbox_dir = temp_dir.path().join("sandbox");
    std::fs::create_dir_all(&sandbox_dir)?;
    
    let sandbox = ArgentSandbox::new();
    sandbox.init(sandbox_dir).await?;
    
    let dir = sandbox.get_directory().await;
    assert!(dir.exists(), "Sandbox directory should exist");
    
    println!("TEST_PASSED: sandbox_directory");
    Ok(())
}

#[tokio::test]
async fn test_sandbox_execution_limits() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let sandbox_dir = temp_dir.path().join("sandbox");
    std::fs::create_dir_all(&sandbox_dir)?;
    
    let sandbox = ArgentSandbox::new();
    sandbox.init(sandbox_dir).await?;
    
    let max_mem = sandbox.get_max_memory().await;
    println!("Max memory: {} bytes", max_mem);
    
    let max_time = sandbox.get_max_execution_time().await;
    println!("Max execution time: {} seconds", max_time);
    
    println!("TEST_PASSED: sandbox_execution_limits");
    Ok(())
}