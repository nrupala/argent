#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use wasmtime::Engine;

#[derive(thiserror::Error, Debug)]
pub enum SandboxError {
    #[error("WASM module error: {0}")]
    ModuleError(String),
    #[error("WASM runtime error: {0}")]
    RuntimeError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Not initialized")]
    NotInitialized,
}

#[derive(Clone)]
pub struct ArgentSandbox {
    inner: Arc<Mutex<ArgentSandboxInner>>,
}

struct ArgentSandboxInner {
    engine: Option<Engine>,
    allowed_dirs: Vec<PathBuf>,
    initialized: bool,
}

impl ArgentSandbox {
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(ArgentSandboxInner::new()));
        ArgentSandbox { inner }
    }

    pub async fn init(&self, allowed_workspace: PathBuf) -> Result<(), SandboxError> {
        let mut sandbox = self.inner.lock().await;
        sandbox.init(allowed_workspace)
    }

    pub async fn is_initialized(&self) -> bool {
        let sandbox = self.inner.lock().await;
        sandbox.initialized
    }

    pub async fn allowed_dirs(&self) -> Vec<PathBuf> {
        let sandbox = self.inner.lock().await;
        sandbox.allowed_dirs.clone()
    }
}

impl Default for ArgentSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl ArgentSandboxInner {
    fn new() -> Self {
        Self {
            engine: None,
            allowed_dirs: Vec::new(),
            initialized: false,
        }
    }

    fn init(&mut self, allowed_workspace: PathBuf) -> Result<(), SandboxError> {
        let engine = Engine::new(
            wasmtime::Config::new()
                .max_wasm_stack(1 << 20)
                .wasm_threads(false)
                .wasm_reference_types(true)
                .wasm_bulk_memory(true)
                .wasm_multi_value(true)
        )
        .map_err(|e| SandboxError::ModuleError(format!("Engine creation failed: {}", e)))?;

        self.engine = Some(engine);
        self.allowed_dirs = vec![allowed_workspace];
        self.initialized = true;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_initialization() {
        let sandbox = ArgentSandbox::new();
        let temp_dir = std::env::temp_dir().join("argent_sandbox_test");
        
        let result = sandbox.init(temp_dir.clone()).await;
        assert!(result.is_ok());
        
        let is_init = sandbox.is_initialized().await;
        assert!(is_init);
    }

    #[tokio::test]
    async fn test_allowed_dirs() {
        let sandbox = ArgentSandbox::new();
        let temp_dir = std::env::temp_dir().join("argent_sandbox_dirs_test");
        
        let _ = sandbox.init(temp_dir.clone()).await;
        
        let dirs = sandbox.allowed_dirs().await;
        assert!(!dirs.is_empty());
    }
}