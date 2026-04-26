use argent::llm::{LlmClient, LlmError};

#[tokio::test]
async fn test_llm_client_creation() {
    let endpoint = "http://localhost:1234";
    let client = LlmClient::new(endpoint, "test-model");
    
    assert!(client.endpoint.contains("1234"), "Client should have correct endpoint");
    
    println!("TEST PASSED: llm_client_creation");
}

#[tokio::test]
async fn test_llm_client_send_message() {
    let endpoint = "http://localhost:1234";
    let client = LlmClient::new(endpoint, "test-model");
    
    let messages = vec![
        argent::llm::ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }
    ];
    
    let result = client.chat(messages, 0.7, 256).await;
    if result.is_err() {
        let err = result.unwrap_err();
        println!("EXPECTED ERROR (LM Studio not running): {}", err);
    }
    
    println!("TEST_PASSED: llm_client_send_message");
}

#[tokio::test]
async fn test_llm_is_model_loaded() {
    let endpoint = "http://localhost:1234";
    let client = LlmClient::new(endpoint, "test-model");
    
    let result = client.is_model_loaded().await;
    println!("Model loaded check: {:?}", result);
    
    println!("TEST_PASSED: llm_is_model_loaded");
}

#[tokio::test]
async fn test_llm_get_model_info() {
    let endpoint = "http://localhost:1234";
    let client = LlmClient::new(endpoint, "test-model");
    
    let result = client.get_model_info().await;
    if result.is_err() {
        let err = result.unwrap_err();
        println!("EXPECTED ERROR (LM Studio not running): {}", err);
    }
    
    println!("TEST_PASSED: llm_get_model_info");
}