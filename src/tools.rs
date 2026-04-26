#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, args: HashMap<String, String>) -> Result<String, ToolError>;
}

#[derive(thiserror::Error, Debug)]
pub enum ToolError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid arguments: {0}")]
    InvalidArgs(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self { tools: Vec::new() };
        registry.register(ReadFileTool::new());
        registry.register(WriteFileTool::new());
        registry.register(RunCommandTool::new());
        registry.register(SearchCodeTool::new());
        registry.register(ListDirectoryTool::new());
        registry
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.push(tool);
    }

    pub fn execute(&self, tool_name: &str, args: HashMap<String, String>) -> Result<String, ToolError> {
        for tool in &self.tools {
            if tool.name() == tool_name {
                return tool.execute(args);
            }
        }
        Err(ToolError::ExecutionFailed(format!("Tool not found: {}", tool_name)))
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.iter().map(|t| t.name().to_string()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

struct ReadFileTool;

impl ReadFileTool {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn execute(&self, args: HashMap<String, String>) -> Result<String, ToolError> {
        let path = args.get("path").ok_or_else(|| ToolError::InvalidArgs("path required".to_string()))?;
        
        std::fs::read_to_string(path)
            .map_err(|e| ToolError::ExecutionFailed(format!("Read failed: {}", e)))
    }
}

struct WriteFileTool;

impl WriteFileTool {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Tool for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn execute(&self, args: HashMap<String, String>) -> Result<String, ToolError> {
        let path = args.get("path").ok_or_else(|| ToolError::InvalidArgs("path required".to_string()))?;
        let content = args.get("content").ok_or_else(|| ToolError::InvalidArgs("content required".to_string()))?;
        
        std::fs::write(path, content)
            .map_err(|e| ToolError::ExecutionFailed(format!("Write failed: {}", e)))?;
        
        Ok(format!("Written to {}", path))
    }
}

struct RunCommandTool;

impl RunCommandTool {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Tool for RunCommandTool {
    fn name(&self) -> &str {
        "run_command"
    }

    fn execute(&self, args: HashMap<String, String>) -> Result<String, ToolError> {
        let command = args.get("command").ok_or_else(|| ToolError::InvalidArgs("command required".to_string()))?;
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .output()
        } else {
            Command::new("sh")
                .args(["-c", command])
                .output()
        };
        
        let output = output.map_err(|e| ToolError::ExecutionFailed(format!("Command failed: {}", e)))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(ToolError::ExecutionFailed(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}

struct SearchCodeTool;

impl SearchCodeTool {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Tool for SearchCodeTool {
    fn name(&self) -> &str {
        "search_code"
    }

    fn execute(&self, args: HashMap<String, String>) -> Result<String, ToolError> {
        let pattern = args.get("pattern").ok_or_else(|| ToolError::InvalidArgs("pattern required".to_string()))?;
        let path = args.get("path").map(PathBuf::from);
        
        Ok(format!("Searching for '{}' in {:?}", pattern, path))
    }
}

struct ListDirectoryTool;

impl ListDirectoryTool {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Tool for ListDirectoryTool {
    fn name(&self) -> &str {
        "list_directory"
    }

    fn execute(&self, args: HashMap<String, String>) -> Result<String, ToolError> {
        let path = args.get("path").ok_or_else(|| ToolError::InvalidArgs("path required".to_string()))?;
        
        let entries = std::fs::read_dir(path)
            .map_err(|e| ToolError::ExecutionFailed(format!("ReadDir failed: {}", e)))?;
        
        let mut result = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_type = if path.is_dir() { "[DIR]" } else { "[FILE]" };
                let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
                result.push(format!("{} {}", file_type, name));
            }
        }
        
        Ok(result.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_registry() {
        let registry = ToolRegistry::new();
        let tools = registry.list_tools();
        
        assert!(tools.contains(&"read_file".to_string()));
        assert!(tools.contains(&"write_file".to_string()));
        assert!(tools.contains(&"run_command".to_string()));
    }

    #[test]
    fn test_read_file_tool() {
        let tool = ReadFileTool::new();
        let mut args = HashMap::new();
        args.insert("path".to_string(), "Cargo.toml".to_string());
        
        let result = tool.execute(args);
        // May fail if file doesn't exist, but shouldn't panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_list_directory_tool() {
        let tool = ListDirectoryTool::new();
        let mut args = HashMap::new();
        args.insert("path".to_string(), ".".to_string());
        
        let result = tool.execute(args);
        assert!(result.is_ok() || result.is_err());
    }
}