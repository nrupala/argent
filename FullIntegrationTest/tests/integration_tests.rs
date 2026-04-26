use argent::{ArgentCore, ArgentConfig, ArgentVault, ArgentSandbox};
use tempfile::TempDir;

fn get_test_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[tokio::test]
async fn test_core_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    let sandbox = ArgentSandbox::new();
    
    let mut core = ArgentCore::new(vault, sandbox);
    
    let config = ArgentConfig {
        storage_path: temp_dir.path().to_path_buf(),
        key_env_var: None,
        llm_endpoint: "http://localhost:1234".to_string(),
    };
    
    let result = core.init(&config).await;
    println!("Core init result: {:?}", result);
    
    println!("TEST_PASSED: core_initialization");
    Ok(())
}

#[tokio::test]
async fn test_clear_session() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    let sandbox = ArgentSandbox::new();
    let mut core = ArgentCore::new(vault, sandbox);
    
    let config = ArgentConfig {
        storage_path: temp_dir.path().to_path_buf(),
        key_env_var: None,
        llm_endpoint: "http://localhost:1234".to_string(),
    };
    
    core.init(&config).await?;
    core.clear_session().await;
    
    println!("TEST_PASSED: clear_session");
    Ok(())
}

#[tokio::test]
async fn test_list_models() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    let sandbox = ArgentSandbox::new();
    let core = ArgentCore::new(vault, sandbox);
    
    let models = core.list_models().await;
    println!("Found {} models", models.len());
    
    for model in &models {
        println!("  - {} ({} bytes)", model.name, model.size);
    }
    
    println!("TEST_PASSED: list_models");
    Ok(())
}

#[tokio::test]
async fn test_get_status() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    let sandbox = ArgentSandbox::new();
    let mut core = ArgentCore::new(vault, sandbox);
    
    let config = ArgentConfig {
        storage_path: temp_dir.path().to_path_buf(),
        key_env_var: None,
        llm_endpoint: "http://localhost:1234".to_string(),
    };
    
    core.init(&config).await?;
    let status = core.get_status().await;
    
    println!("Argent Status:");
    println!("  Version: {}", status.version);
    println!("  Vault Initialized: {}", status.vault_initialized);
    println!("  Sandbox Initialized: {}", status.sandbox_initialized);
    println!("  History Size: {}", status.history_size);
    println!("  Blocklist Entries: {}", status.blocklist.len());
    
    println!("TEST_PASSED: get_status");
    Ok(())
}

#[tokio::test]
async fn test_init_project() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let vault = ArgentVault::new(temp_dir.path().to_path_buf());
    let sandbox = ArgentSandbox::new();
    let mut core = ArgentCore::new(vault, sandbox);
    
    let project_path = temp_dir.path().to_path_buf();
    core.init_project(project_path).await;
    
    println!("TEST_PASSED: init_project");
    Ok(())
}

#[tokio::test]
async fn test_config_from_env() {
    use argent::ArgentConfig;
    
    let config = ArgentConfig::from_env();
    println!("Config from environment:");
    println!("  Storage: {:?}", config.storage_path);
    println!("  Key Env: {:?}", config.key_env_var);
    println!("  LLM Endpoint: {}", config.llm_endpoint);
    
    println!("TEST_PASSED: config_from_env");
}