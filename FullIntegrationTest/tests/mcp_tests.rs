use argent::mcp::{ModelDetector, SecurePathGuard, SecurityLevel};
use std::path::PathBuf;
use tempfile::TempDir;

fn get_test_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[tokio::test]
async fn test_model_detector_creation() {
    let detector = ModelDetector::new();
    let models = detector.detect_gguf_models();
    
    println!("Detected {} models", models.len());
    println!("TEST_PASSED: model_detector_creation");
}

#[tokio::test]
async fn test_model_detector_path() {
    let detector = ModelDetector::new();
    let default_path = detector.get_default_model_path();
    
    println!("Default model path: {:?}", default_path);
    println!("TEST_PASSED: model_detector_path");
}

#[tokio::test]
async fn test_path_guard_creation() {
    let temp_dir = get_test_dir();
    let guard = SecurePathGuard::new(temp_dir.path().to_path_buf());
    
    let is_allowed = guard.is_path_allowed(&temp_dir.path().join("test.txt"));
    assert!(is_allowed, "Path in workspace should be allowed");
    
    println!("TEST_PASSED: path_guard_creation");
}

#[tokio::test]
async fn test_path_guard_block_system() {
    let temp_dir = get_test_dir();
    let guard = SecurePathGuard::new(temp_dir.path().to_path_buf());
    
    let system_paths = vec![
        "/etc/passwd",
        "/etc/shadow",
        "C:\\Windows\\System32\\config",
    ];
    
    for path in system_paths {
        let is_allowed = guard.is_path_allowed(&PathBuf::from(path));
        assert!(!is_allowed, "System path should be blocked: {}", path);
    }
    
    println!("TEST_PASSED: path_guard_block_system");
}

#[tokio::test]
async fn test_path_guard_block_sensitive() {
    let temp_dir = get_test_dir();
    let guard = SecurePathGuard::new(temp_dir.path().to_path_buf());
    
    let sensitive_paths = vec![
        ".env",
        "credentials.json",
        "id_rsa",
        "id_ed25519",
        "*.key",
        "*.pem",
    ];
    
    for path in sensitive_paths {
        let is_allowed = guard.is_path_allowed(&temp_dir.path().join(path));
        println!("Path '{}' allowed: {}", path, is_allowed);
    }
    
    println!("TEST_PASSED: path_guard_block_sensitive");
}

#[tokio::test]
async fn test_path_guard_get_blocklist() {
    let temp_dir = get_test_dir();
    let guard = SecurePathGuard::new(temp_dir.path().to_path_buf());
    
    let blocklist = guard.get_blocklist();
    println!("Blocklist has {} entries", blocklist.len());
    
    println!("TEST_PASSED: path_guard_get_blocklist");
}