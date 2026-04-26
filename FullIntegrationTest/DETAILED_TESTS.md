# Test 1: Crypto Vault Test
**Module:** crypto
**Test:** test_encrypt_decrypt_roundtrip
**Status:** ✅ PASSED

## Example
```rust
let vault = ArgentVault::new(temp_path);
vault.initialize(None).await?;
let encrypted = vault.encrypt("Hello Argent").await?;
let decrypted = vault.decrypt(&encrypted).await?;
assert_eq!(decrypted, "Hello Argent");
```

## Functionality Tested
- XChaCha20-Poly1305 encryption
- Key derivation from optional master key
- Encryption/decryption roundtrip

---
# Test 2: Crypto Concurrent Access Test
**Module:** crypto
**Test:** test_concurrent_access
**Status:** ✅ PASSED

## Example
```rust
let vault = ArgentVault::new(temp_path);
vault.initialize(None).await?;
let vault_clone = vault.clone();
let handle = tokio::spawn(async move {
    vault_clone.encrypt("test").await
});
// Concurrent operations work
```

## Functionality Tested
- Async-safe Arc<Mutex> wrapper
- Concurrent encryption operations
- Thread safety of vault operations

---
# Test 3: Crypto State Persistence Test
**Module:** crypto
**Test:** test_state_persistence
**Status:** ⚠️ IGNORED (Known Issue)

## Example
```rust
vault.store_state("session", data).await?;
let loaded = vault.load_state("session").await?;
```

## Functionality Tested
- State persistence to disk (Currently investigating issue)

---
# Test 4: Tools Registry Test
**Module:** tools
**Test:** test_tool_registry
**Status:** ✅ PASSED

## Example
```rust
let registry = ToolRegistry::new();
let tools = registry.list_tools();
// ["read_file", "write_file", "run_command", "search_code", "list_directory"]
```

## Functionality Tested
- Tool registration system
- 5 default tools: read_file, write_file, run_command, search_code, list_directory

---
# Test 5: Tools Read File Test
**Module:** tools
**Test:** test_read_file_tool
**Status:** ✅ PASSED

## Example
```rust
let mut args = HashMap::new();
args.insert("path".to_string(), "test.txt".to_string());
let result = registry.execute("read_file", args)?;
```

## Functionality Tested
- File reading with security validation
- Error handling for missing files
- Path validation

---
# Test 6: Tools List Directory Test
**Module:** tools
**Test:** test_list_directory_tool
**Status:** ✅ PASSED

## Example
```rust
let mut args = HashMap::new();
args.insert("path".to_string(), ".".to_string());
let result = registry.execute("list_directory", args)?;
```

## Functionality Tested
- Directory listing functionality
- Recursive option support

---
# Test 7: Tools Blocklist Validation
**Module:** tools
**Test:** test_tool_blocklist_validation
**Status:** ✅ PASSED

## Example
```rust
let mut args = HashMap::new();
args.insert("command".to_string(), "rm -rf /".to_string());
let result = registry.execute("run_command", args);
// Command blocked due to security policy
```

## Functionality Tested
- Dangerous command blocking
- Security validation

---
# Test 8: LLM Client Creation Test
**Module:** llm
**Test:** test_client_creation
**Status:** ✅ PASSED

## Example
```rust
let client = LlmClient::new("http://localhost:1234", "model-name");
assert!(client.endpoint.contains("1234"));
```

## Functionality Tested
- HTTP client initialization
- Endpoint configuration
- Model name storage

---
# Test 9: LLM Chat Request Serialization
**Module:** llm
**Test:** test_chat_request_serialization
**Status:** ✅ PASSED

## Example
```rust
let request = ChatRequest {
    model: "test-model".to_string(),
    messages: vec![ChatMessage { role: "user".to_string(), content: "Hello".to_string() }],
    temperature: 0.7,
    max_tokens: 256,
    stream: false,
};
let json = serde_json::to_string(&request)?;
```

## Functionality Tested
- JSON serialization for LM Studio API
- Request format compatibility

---
# Test 10: Sandbox Initialization
**Module:** sandbox
**Test:** test_sandbox_initialization
**Status:** ✅ PASSED

## Example
```rust
let sandbox = ArgentSandbox::new();
sandbox.init(sandbox_dir).await?;
assert!(sandbox.is_initialized().await);
```

## Functionality Tested
- Wasmtime engine initialization
- Sandbox directory creation
- Memory limit configuration

---
# Test 11: Sandbox Allowed Directories
**Module:** sandbox
**Test:** test_allowed_dirs
**Status:** ✅ PASSED

## Example
```rust
let sandbox = ArgentSandbox::new();
let allowed = sandbox.get_allowed_dirs().await;
assert!(allowed.contains(&workspace_path));
```

## Functionality Tested
- Allowed directory configuration
- WASM execution boundary

---
# Test 12: MCP Model Detector
**Module:** mcp
**Test:** test_model_detector_creation
**Status:** ✅ PASSED

## Example
```rust
let detector = ModelDetector::new();
let models = detector.detect_gguf_models();
println!("Found {} GGUF models", models.len());
```

## Functionality Tested
- GGUF model auto-detection
- Model path scanning
- File size calculation

---
# Test 13: MCP Path Guard
**Module:** mcp
**Test:** test_path_guard
**Status:** ✅ PASSED

## Example
```rust
let guard = SecurePathGuard::new(workspace.clone());
assert!(!guard.is_path_allowed(&PathBuf::from("/etc/passwd")));
assert!(guard.is_path_allowed(&workspace.join("file.txt")));
```

## Functionality Tested
- System path blocking
- Workspace path allowance
- Security blocklist enforcement

---
# Test 14: MCP Model Type Guess
**Module:** mcp
**Test:** test_model_type_guess
**Status:** ✅ PASSED

## Example
```rust
let detector = ModelDetector::new();
let info = detector.guess_model_info("Llama-3-8B-Q4_K_M.gguf");
assert!(info.name.contains("Llama"));
```

## Functionality Tested
- Model name parsing
- Size-based model type guessing

---
# Test 15: Prompts Thought Creation
**Module:** prompts
**Test:** test_thought_creation
**Status:** ✅ PASSED

## Example
```rust
let thought = Thought::new("Thinking about solution...".to_string());
assert!(thought.observation.is_none());
```

## Functionality Tested
- ReAct thought structure creation
- Observation field management

---
# Test 16: Prompts Thought with Action
**Module:** prompts
**Test:** test_thought_with_action
**Status:** ✅ PASSED

## Example
```rust
let mut thought = Thought::new("Let me check the file".to_string());
thought.action = Some(ToolAction {
    tool: "read_file".to_string(),
    args: HashMap::new(),
});
```

## Functionality Tested
- Action attachment to thoughts
- Tool action structure

---
# Test 17: Prompts Context Building
**Module:** prompts
**Test:** test_build_context
**Status:** ✅ PASSED

## Example
```rust
let history = vec![Thought::new("Hello".to_string())];
let context = build_context(history, 3);
assert!(context.contains("Hello"));
```

## Functionality Tested
- History to context conversion
- Token limit handling
- System prompt integration