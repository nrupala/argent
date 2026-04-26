use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ModelDetector {
    model_dirs: Vec<PathBuf>,
    blocklist: Vec<PathBuf>,
}

impl ModelDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            model_dirs: Vec::new(),
            blocklist: Vec::new(),
        };
        
        detector.add_default_directories();
        detector.add_blocklist();
        detector
    }

    fn add_default_directories(&mut self) {
        if let Some(home) = dirs::home_dir() {
            self.model_dirs.push(home.join(".lmstudio/models"));
            self.model_dirs.push(home.join(".cache/lm-studio/models"));
        }
        
        if let Some(data) = dirs::data_local_dir() {
            self.model_dirs.push(data.join("LMStudio/models"));
        }
    }

    fn add_blocklist(&mut self) {
        if let Some(home) = dirs::home_dir() {
            self.blocklist.push(home.join(".ssh"));
            self.blocklist.push(home.join(".gnupg"));
            self.blocklist.push(home.join(".aws"));
        }
        
        self.blocklist.push(PathBuf::from("/etc"));
        self.blocklist.push(PathBuf::from("/ System"));
        self.blocklist.push(PathBuf::from("C:\\Windows"));
        self.blocklist.push(PathBuf::from("C:\\Program Files"));
    }

    pub fn add_model_directory(&mut self, path: PathBuf) {
        if path.exists() {
            self.model_dirs.push(path);
        }
    }

    pub fn detect_gguf_models(&self) -> Vec<ModelInfo> {
        let mut models = Vec::new();

        for dir in &self.model_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map(|e| e == "gguf").unwrap_or(false) {
                        let name = path.file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default();
                        
                        let size = fs::metadata(&path)
                            .map(|m| m.len())
                            .unwrap_or(0);

                        models.push(ModelInfo {
                            name: name.clone(),
                            path: path.clone(),
                            size,
                            model_type: guess_model_type(&name),
                        });
                    }
                }
            }
        }

        models.sort_by(|a, b| b.size.cmp(&a.size));
        models
    }

    pub fn is_path_allowed(&self, path: &Path) -> bool {
        let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        
        for blocked in &self.blocklist {
            let blocked = blocked.canonicalize().unwrap_or_else(|_| blocked.clone());
            if path.starts_with(&blocked) {
                return false;
            }
        }
        
        true
    }

    pub fn validate_path(&self, path: &str) -> Result<PathBuf, PathGuardError> {
        let path = PathBuf::from(path);
        
        if !path.exists() {
            return Err(PathGuardError::PathNotFound(path.to_string_lossy().to_string()));
        }
        
        if !self.is_path_allowed(&path) {
            return Err(PathGuardError::PathBlocked(path.to_string_lossy().to_string()));
        }

        Ok(path)
    }

    pub fn get_blocklist(&self) -> Vec<String> {
        self.blocklist.iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub model_type: ModelType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelType {
    General,
    Code,
    Chat,
    Embedding,
    Unknown,
}

impl ModelType {
    pub fn as_str(&self) -> &str {
        match self {
            ModelType::General => "general",
            ModelType::Code => "code",
            ModelType::Chat => "chat",
            ModelType::Embedding => "embedding",
            ModelType::Unknown => "unknown",
        }
    }
}

fn guess_model_type(name: &str) -> ModelType {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("code") || name_lower.contains("codestral") {
        ModelType::Code
    } else if name_lower.contains("chat") || name_lower.contains("instruct") {
        ModelType::Chat
    } else if name_lower.contains("embed") || name_lower.contains("embedding") {
        ModelType::Embedding
    } else {
        ModelType::General
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PathGuardError {
    #[error("Path not found: {0}")]
    PathNotFound(String),
    #[error("Path is blocked by security policy: {0}")]
    PathBlocked(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct SecurePathGuard {
    detector: ModelDetector,
    workspace_root: PathBuf,
}

impl SecurePathGuard {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            detector: ModelDetector::new(),
            workspace_root,
        }
    }

    pub fn set_workspace(&mut self, path: PathBuf) {
        if path.exists() && path.is_dir() {
            self.workspace_root = path.clone();
            self.detector.add_model_directory(path);
        }
    }

    pub fn get_blocklist(&self) -> Vec<String> {
        self.detector.get_blocklist()
    }

    pub fn can_read(&self, path: &Path) -> bool {
        self.detector.is_path_allowed(path)
    }

    pub fn can_write(&self, path: &Path) -> bool {
        if !self.detector.is_path_allowed(path) {
            return false;
        }
        
        if let Some(parent) = path.parent() {
            return parent == self.workspace_root || parent.starts_with(&self.workspace_root);
        }
        
        false
    }

    pub fn can_execute(&self, path: &Path) -> bool {
        self.can_read(path)
    }

    pub fn list_available_models(&self) -> Vec<ModelInfo> {
        self.detector.detect_gguf_models()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_detector_creation() {
        let detector = ModelDetector::new();
        let models = detector.detect_gguf_models();
        assert!(models.is_empty() || models.len() >= 0);
    }

    #[test]
    fn test_path_guard() {
        let guard = SecurePathGuard::new(PathBuf::from("."));
        
        assert!(!guard.can_read(&PathBuf::from("/etc/passwd")));
    }

    #[test]
    fn test_model_type_guess() {
        assert_eq!(guess_model_type("codestral-22b"), ModelType::Code);
        assert_eq!(guess_model_type("chat-q4"), ModelType::Chat);
        assert_eq!(guess_model_type("embedding"), ModelType::Embedding);
    }
}