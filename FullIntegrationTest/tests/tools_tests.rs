use argent::tools::{ToolRegistry, ToolError};
use std::collections::HashMap;
use tempfile::TempDir;

fn get_test_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[tokio::test]
async fn test_tool_registry_list() {
    let registry = ToolRegistry::new();
    let tools = registry.list_tools();
    
    let expected_tools = vec![
        "read_file".to_string(),
        "write_file".to_string(),
        "run_command".to_string(),
        "search_code".to_string(),
        "list_directory".to_string(),
    ];
    
    for tool in expected_tools {
        assert!(tools.contains(&tool), "Registry should contain tool: {}", tool);
    }
    
    println!("TEST PASSED: tool_registry_list");
}

#[tokio::test]
async fn test_read_file_tool() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "Hello, World!")?;
    
    let mut args = HashMap::new();
    args.insert("path".to_string(), test_file.to_string_lossy().to_string());
    
    let registry = ToolRegistry::new();
    let result = registry.execute("read_file", args);
    assert!(result.is_ok(), "Read file should succeed");
    
    let content = result.unwrap();
    assert!(content.contains("Hello"), "Content should match");
    
    println!("TEST PASSED: read_file_tool");
    Ok(())
}

#[tokio::test]
async fn test_write_file_tool() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = get_test_dir();
    let test_file = temp_dir.path().join("output.txt");
    let content = "Test content for write";
    
    let mut args = HashMap::new();
    args.insert("path".to_string(), test_file.to_string_lossy().to_string());
    args.insert("content".to_string(), content.to_string());
    
    let registry = ToolRegistry::new();
    let result = registry.execute("write_file", args);
    assert!(result.is_ok(), "Write file should succeed");
    
    let written = std::fs::read_to_string(&test_file)?;
    assert_eq!(written, content, "Written content should match");
    
    println!("TEST PASSED: write_file_tool");
    Ok(())
}

#[tokio::test]
async fn test_run_command_tool() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = HashMap::new();
    #[cfg(windows)]
    args.insert("command".to_string(), "echo test_command".to_string());
    #[cfg(not(windows))]
    args.insert("command".to_string(), "echo test_command".to_string());
    
    let registry = ToolRegistry::new();
    let result = registry.execute("run_command", args);
    assert!(result.is_ok(), "Run command should succeed");
    
    println!("TEST PASSED: run_command_tool");
    Ok(())
}

#[tokio::test]
async fn test_tool_not_found() {
    let registry = ToolRegistry::new();
    let mut args = HashMap::new();
    args.insert("path".to_string(), "test".to_string());
    
    let result = registry.execute("nonexistent_tool", args);
    assert!(result.is_err(), "Nonexistent tool should fail");
    
    println!("TEST PASSED: tool_not_found");
}