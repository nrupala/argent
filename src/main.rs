#![allow(dead_code)]

use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

pub mod crypto;
pub mod sandbox;
pub mod prompts;
pub mod tools;
pub mod llm;
pub mod mcp;

use crypto::{ArgentVault, VaultError};
use sandbox::{ArgentSandbox, SandboxError};
use prompts::Thought;
use tools::ToolRegistry;
use llm::{LlmClient, LlmError};
use mcp::{ModelDetector, SecurePathGuard, ModelInfo};
use thiserror::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
pub enum ArgentError {
    #[error("Vault error: {0}")]
    Vault(#[from] VaultError),
    #[error("Tool error: {0}")]
    Tool(#[from] tools::ToolError),
    #[error("Sandbox error: {0}")]
    Sandbox(#[from] SandboxError),
    #[error("LLM error: {0}")]
    Llm(#[from] LlmError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

struct ArgentCore {
    vault: ArgentVault,
    sandbox: ArgentSandbox,
    tools: ToolRegistry,
    path_guard: SecurePathGuard,
    model_detector: ModelDetector,
    history: Arc<Mutex<Vec<Thought>>>,
    llm: Option<LlmClient>,
    running: Arc<Mutex<bool>>,
    current_model: Option<ModelInfo>,
}

impl ArgentCore {
    fn new(vault: ArgentVault, sandbox: ArgentSandbox) -> Self {
        Self {
            vault,
            sandbox,
            tools: ToolRegistry::new(),
            path_guard: SecurePathGuard::new(PathBuf::from(".")),
            model_detector: ModelDetector::new(),
            history: Arc::new(Mutex::new(Vec::new())),
            llm: None,
            running: Arc::new(Mutex::new(false)),
            current_model: None,
        }
    }

    async fn init(&mut self, config: &ArgentConfig) -> Result<(), ArgentError> {
        let master_key: Option<Vec<u8>> = if let Some(ref env_var) = config.key_env_var {
            let key = env::var(env_var).ok();
            if key.as_ref().map(|k| k.len() == 32).unwrap_or(false) {
                Some(key.unwrap().into_bytes())
            } else {
                None
            }
        } else {
            None
        };

        self.vault.initialize(master_key.as_deref()).await.map_err(ArgentError::Vault)?;
        
        let storage = self.vault.storage_path().await;
        
        let sandbox_dir = storage.join("sandbox");
        self.sandbox.init(sandbox_dir).await.map_err(ArgentError::Sandbox)?;
        
        let detected_models = self.model_detector.detect_gguf_models();
        
        let model_info = if !detected_models.is_empty() {
            Some(detected_models.into_iter().next().unwrap())
        } else {
            None
        };

        if let Some(ref model) = model_info {
            println!("[Argent-v{}] Auto-detected model: {} ({:.1}GB)", 
                VERSION,
                model.name,
                model.size as f64 / 1_073_741_824.0
            );
            
            self.llm = Some(LlmClient::new(&config.llm_endpoint, &model.name));
            self.current_model = Some(model.clone());
        } else {
            println!("[Argent-v{}] No GGUF models found", VERSION);
            println!("[Argent] Add --init to scan for models");
        }
        
        println!("[Argent] Tools: {:?}", self.tools.list_tools());
        
        Ok(())
    }

    pub async fn init_project(&mut self, path: PathBuf) {
        self.path_guard.set_workspace(path.clone());
        
        let models = self.model_detector.detect_gguf_models();
        
        if let Some(model) = models.into_iter().next() {
            self.llm = Some(LlmClient::new("http://localhost:1234", &model.name));
            self.current_model = Some(model.clone());
            
            println!("[Argent] Project initialized with model: {}", model.name);
        }
    }

    pub async fn clear_session(&self) {
        let mut history = self.history.lock().await;
        history.clear();
        
        if let Err(e) = self.vault.store_state("_session", "{}").await {
            eprintln!("[Argent] Clear error: {}", e);
        }
        
        println!("[Argent] Session cleared");
    }

    pub async fn list_models(&self) -> Vec<ModelInfo> {
        self.model_detector.detect_gguf_models()
    }

    pub async fn get_status(&self) -> ArgentStatus {
        let history = self.history.lock().await;
        
        ArgentStatus {
            version: VERSION.to_string(),
            vault_initialized: self.vault.is_initialized().await,
            sandbox_initialized: self.sandbox.is_initialized().await,
            current_model: self.current_model.clone(),
            history_size: history.len(),
            blocklist: self.path_guard.get_blocklist(),
        }
    }

    async fn run(&mut self) -> Result<(), ArgentError> {
        {
            let mut running = self.running.lock().await;
            *running = true;
        }

        println!("\n===========================================");
        println!("  Argent v{} - Zero-Knowledge Engine", VERSION);
        println!("===========================================");
        println!();
        println!("Commands:");
        println!("  /init <path>  - Initialize project workspace");
        println!("  /models       - List available GGUF models");
        println!("  /clear        - Clear session history");
        println!("  /status       - Show system status");
        println!("  /help         - Show this help");
        println!("  Ctrl+C        - Shutdown");
        println!();

        loop {
            let is_running = {
                let running = self.running.lock().await;
                *running
            };
            
            if !is_running {
                break;
            }

            sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }

    async fn shutdown(&self) {
        let mut running = self.running.lock().await;
        *running = false;

        if let Err(e) = self.vault.store_state("_session", r#"{"status":"shutdown"}"#).await {
            eprintln!("[Argent] Flush error: {}", e);
        }

        println!("[Argent] Shutdown complete.");
    }
}

#[derive(Debug, Clone)]
pub struct ArgentStatus {
    pub version: String,
    pub vault_initialized: bool,
    pub sandbox_initialized: bool,
    pub current_model: Option<ModelInfo>,
    pub history_size: usize,
    pub blocklist: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ArgentConfig {
    pub storage_path: PathBuf,
    pub key_env_var: Option<String>,
    pub llm_endpoint: String,
}

impl Default for ArgentConfig {
    fn default() -> Self {
        Self {
            storage_path: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("argent"),
            key_env_var: Some("ARGENT_KEY".to_string()),
            llm_endpoint: "http://localhost:1234".to_string(),
        }
    }
}

impl ArgentConfig {
    fn from_env() -> Self {
        Self {
            storage_path: env::var("ARGENT_STORAGE")
                .map(PathBuf::from)
                .unwrap_or_else(|_| Self::default().storage_path),
            key_env_var: env::var("ARGENT_KEY_VAR").ok(),
            llm_endpoint: env::var("ARGENT_LLM_ENDPOINT")
                .unwrap_or_else(|_| Self::default().llm_endpoint),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ArgentError> {
    let config = ArgentConfig::from_env();
    
    let vault = ArgentVault::new(config.storage_path.clone());
    let sandbox = ArgentSandbox::new();
    let mut core = ArgentCore::new(vault, sandbox);
    
    core.init(&config).await?;
    core.run().await?;
    core.shutdown().await;
    
    Ok(())
}