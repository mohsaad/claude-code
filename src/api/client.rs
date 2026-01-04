use anyhow::{Context, Result};
use reqwest::Client;

use super::types::{ContentBlock, Message, MessageContent, MessageRequest, MessageResponse, Tool};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MODEL: &str = "claude-3-5-sonnet-20241022";
const MAX_TOKENS: u32 = 4096;

const SYSTEM_MESSAGE: &str = "You are a helpful coding assistant. You have access to tools to read, write, and edit files, list directories, and execute shell commands. Always explain what you're doing before using tools. Be concise and focused on solving the user's coding tasks.";

pub struct ClaudeClient {
    client: Client,
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageResponse> {
        let request = MessageRequest {
            model: DEFAULT_MODEL.to_string(),
            max_tokens: MAX_TOKENS,
            messages,
            tools,
            system: Some(SYSTEM_MESSAGE.to_string()),
        };

        let response = self
            .client
            .post(API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("API request failed with status {}: {}", status, error_text);
        }

        let message_response: MessageResponse = response
            .json()
            .await
            .context("Failed to parse Claude API response")?;

        Ok(message_response)
    }

    pub async fn simple_prompt(&self, prompt: &str) -> Result<String> {
        let message = Message {
            role: "user".to_string(),
            content: MessageContent::Text(prompt.to_string()),
        };

        let response = self.send_message(vec![message], None).await?;

        // Extract text from response
        let mut result = String::new();
        for block in response.content {
            if let ContentBlock::Text { text } = block {
                result.push_str(&text);
            }
        }

        Ok(result)
    }
}
