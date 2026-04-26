# Argent Full Integration Test Results

**Date:** 2026-04-26
**Version:** 0.1.0

## Test Summary

| Category | Passed | Failed | Ignored | Total |
|----------|--------|--------|--------|-------|
| Crypto Tests | 3 | 0 | 1 | 4 |
| Tools Tests | 4 | 0 | 0 | 4 |
| LLM Tests | 2 | 0 | 0 | 2 |
| Sandbox Tests | 2 | 0 | 0 | 2 |
| MCP Tests | 3 | 0 | 0 | 3 |
| Integration Tests | 2 | 0 | 0 | 2 |
| **TOTAL** | **16** | **0** | **1** | **17** |

## Test Results Detail

### Crypto Module (4 tests)
- ✅ `test_encrypt_decrypt_roundtrip` - Encrypt/decrypt cycle works correctly
- ✅ `test_concurrent_access` - Concurrent vault access is thread-safe
- ⚠️ `test_state_persistence` - **IGNORED** (Known issue - investigating state persistence)
- ✅ `test_vault_initialization` - Vault initializes correctly

### Tools Module (4 tests)
- ✅ `test_tool_registry` - Tool registry contains all expected tools
- ✅ `test_read_file_tool` - Read file tool works correctly
- ✅ `test_list_directory_tool` - List directory tool works correctly
- ✅ `test_tool_blocklist_validation` - Security blocklist is enforced

### LLM Module (2 tests)
- ✅ `test_client_creation` - LM Studio client creates successfully
- ✅ `test_chat_request_serialization` - Chat request serialization works

### Sandbox Module (2 tests)
- ✅ `test_sandbox_initialization` - WASM sandbox initializes correctly
- ✅ `test_allowed_dirs` - Allowed directories configuration works

### MCP Module (3 tests)
- ✅ `test_model_detector_creation` - Model detector creates successfully
- ✅ `test_path_guard` - Path guard blocks system paths correctly
- ✅ `test_model_type_guess` - Model type detection works

### Prompts Module (4 tests)
- ✅ `test_thought_creation` - ReAct thought creation works
- ✅ `test_thought_with_action` - Thought with action works
- ✅ `test_system_prompt` - System prompt generation works
- ✅ `test_build_context` - Context building works

### Integration Test Summary
- All core components can initialize together
- Vault encrypt/decrypt cycle verified
- Tool registry functional with 5 tools registered
- WASM sandbox functional
- Path guards active for security

## Notes
- 1 test ignored due to state persistence (known issue, under investigation)
- All other tests pass successfully
- Binary location: `target/release/argent.exe`
- Docker image: `argent:latest`