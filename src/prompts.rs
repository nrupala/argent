use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub id: u64,
    pub reasoning: String,
    pub action: Option<Action>,
    pub observation: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub tool: String,
    pub args: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReActStep {
    pub thought: String,
    pub action: Option<Action>,
    pub observation: Option<String>,
}

impl Thought {
    pub fn new(id: u64, reasoning: String) -> Self {
        Self {
            id,
            reasoning,
            action: None,
            observation: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    pub fn with_action(mut self, tool: String, args: HashMap<String, String>) -> Self {
        self.action = Some(Action { tool, args });
        self
    }

    pub fn with_observation(mut self, obs: String) -> Self {
        self.observation = Some(obs);
        self
    }
}

pub struct ArgentPrompt;

impl ArgentPrompt {
    pub fn system_prompt() -> String {
        r#"You are Argent, a self-correcting autonomous engineer.

## Core Directives
- Identity: Argent
- Mission: Help the user build, analyze, and improve software systems
- Security Priority: Always consider security implications of changes
- Zero-Knowledge: Never log secrets, keys, or credentials

## Reasoning Pattern (ReAct)
Think step-by-step:
1. THINK: Analyze the request and plan your approach
2. ACT: Execute the appropriate tool with parameters
3. OBSERVE: Analyze the result and determine next steps
4. If failed, generate a fix and retry

## Available Tools
- read_file: Read file contents
- write_file: Create or overwrite a file
- run_command: Execute a shell command
- search_code: Search for patterns in code
- list_directory: List directory contents

## Code Quality Standards
- Write testable, modular code
- Follow language idioms and best practices
- Include safety checks and error handling
- No secrets or keys in code

## Response Format
When reasoning, output:
[THINK] Your analysis and plan
[ACT] ToolName({ "arg": "value" })
[OBSERVE] The result of your action
"#.to_string()
    }

    pub fn tool_definitions() -> String {
        r#"## Tool Definitions

### read_file
Read file contents
Parameters: path (string, required)
Returns: file contents or error

### write_file
Create or overwrite a file
Parameters: path (string, required), content (string, required)
Returns: success or error

### run_command
Execute a shell command
Parameters: command (string, required), cwd (string, optional)
Returns: stdout/stderr output

### search_code
Search for patterns in code
Parameters: pattern (string, required), path (string, optional)
Returns: matching lines

### list_directory
List directory contents
Parameters: path (string, required)
Returns: file listing
"#.to_string()
    }

    pub fn build_context(history: &[Thought], current: &str) -> String {
        let mut ctx = Self::system_prompt();
        ctx.push_str("\n\n## Conversation History\n");
        
        for thought in history.iter().rev().take(10) {
            ctx.push_str(&format!("\n[Step {}]\n", thought.id));
            ctx.push_str(&format!("THINK: {}\n", thought.reasoning));
            if let Some(ref action) = thought.action {
                ctx.push_str(&format!("ACT: {} {:?}\n", action.tool, action.args));
            }
            if let Some(ref obs) = thought.observation {
                ctx.push_str(&format!("OBSERVE: {}\n", obs));
            }
        }
        
        ctx.push_str(&format!("\n## Current Task\n{}\n", current));
        ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thought_creation() {
        let thought = Thought::new(1, "Analyzing the request".to_string());
        assert_eq!(thought.id, 1);
        assert_eq!(thought.reasoning, "Analyzing the request");
    }

    #[test]
    fn test_thought_with_action() {
        let mut args = HashMap::new();
        args.insert("path".to_string(), "test.rs".to_string());
        
        let thought = Thought::new(1, "Reading file".to_string())
            .with_action("read_file".to_string(), args);
        
        assert!(thought.action.is_some());
        assert_eq!(thought.action.as_ref().unwrap().tool, "read_file");
    }

    #[test]
    fn test_system_prompt() {
        let prompt = ArgentPrompt::system_prompt();
        assert!(prompt.contains("Argent"));
        assert!(prompt.contains("ReAct"));
    }

    #[test]
    fn test_build_context() {
        let history = vec![
            Thought::new(1, "First thought".to_string()),
        ];
        
        let ctx = ArgentPrompt::build_context(&history, "Current task");
        assert!(ctx.contains("First thought"));
        assert!(ctx.contains("Current task"));
    }
}