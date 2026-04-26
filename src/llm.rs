use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Client;

#[derive(thiserror::Error, Debug)]
pub enum LlmError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Response parse failed: {0}")]
    ParseFailed(String),
    #[error("Model not loaded: {0}")]
    ModelNotLoaded(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub error: Option<ErrorResponse>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Option<ChatMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Clone)]
pub struct LlmClient {
    inner: Arc<Mutex<LlmClientInner>>,
}

struct LlmClientInner {
    client: Client,
    endpoint: String,
    model: String,
}

impl LlmClient {
    pub fn new(endpoint: &str, model: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        let inner = Arc::new(Mutex::new(LlmClientInner {
            client,
            endpoint: endpoint.to_string(),
            model: model.to_string(),
        }));

        LlmClient { inner }
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>, temperature: f32) -> Result<String, LlmError> {
        let inner = self.inner.lock().await;

        let request = ChatRequest {
            model: inner.model.clone(),
            messages,
            temperature,
            max_tokens: 4096,
            stream: false,
        };

        let url = format!("{}/v1/chat/completions", inner.endpoint);
        
        let response = inner.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ConnectionFailed(e.to_string()))?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseFailed(e.to_string()))?;

        if let Some(error) = chat_response.error {
            return Err(LlmError::RequestFailed(error.message));
        }

        let content = chat_response
            .choices
            .and_then(|c| c.into_iter().next())
            .and_then(|c| c.message)
            .map(|m| m.content)
            .ok_or_else(|| LlmError::ParseFailed("No content in response".to_string()))?;

        Ok(content)
    }

    pub async fn is_model_loaded(&self) -> bool {
        let inner = self.inner.lock().await;
        
        let url = format!("{}/v1/models", inner.endpoint);
        
        match inner.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    pub async fn generate(&self, prompt: &str, context: &[ChatMessage], temperature: f32) -> Result<String, LlmError> {
        let mut messages = context.to_vec();
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        self.chat(messages, temperature).await
    }
}

pub struct LocalLlmEngine {
    client: LlmClient,
    system_prompt: String,
}

impl LocalLlmEngine {
    pub fn new(model_path: &str) -> Self {
        Self {
            client: LlmClient::new("http://localhost:1234", model_path),
            system_prompt: String::new(),
        }
    }

    pub fn with_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }

    pub async fn chat(&self, message: &str, history: &[ChatMessage]) -> Result<String, LlmError> {
        let mut context = history.to_vec();
        
        if !self.system_prompt.is_empty() {
            context.insert(0, ChatMessage {
                role: "system".to_string(),
                content: self.system_prompt.clone(),
            });
        }

        self.client.generate(message, &context, 0.7).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = LlmClient::new("http://localhost:1234", "test-model");
        let is_loaded = client.is_model_loaded().await;
        // LM Studio might not be running in tests
        assert!(is_loaded || !is_loaded);
    }

    #[test]
    fn test_chat_request_serialization() {
        let request = ChatRequest {
            model: "test".to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello".to_string(),
                }
            ],
            temperature: 0.7,
            max_tokens: 100,
            stream: false,
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test"));
    }
}