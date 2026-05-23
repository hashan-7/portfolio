use super::prompt::build_system_prompt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{env, time::Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct AiClient;

impl AiClient {
    pub async fn get_reply(history: &[ChatMessage], profile_json: &str) -> String {
        let system_prompt = build_system_prompt(profile_json);

        let hf_token = match env::var("HF_API_TOKEN") {
            Ok(token) if !token.trim().is_empty() => token,
            _ => {
                return "[Error] HF_API_TOKEN is missing. Please check your environment configuration."
                    .to_string()
            }
        };

        let hf_model = match env::var("HF_MODEL_ID") {
            Ok(model) if !model.trim().is_empty() => model,
            _ => {
                return "[Error] HF_MODEL_ID is missing. Please check your environment configuration."
                    .to_string()
            }
        };

        if !hf_token.starts_with("hf_") {
            return "[Error] HF_API_TOKEN has an invalid format.".to_string();
        }

        let client = match Client::builder().timeout(Duration::from_secs(60)).build() {
            Ok(client) => client,
            Err(error) => return format!("[Error] Failed to create HTTP client: {}", error),
        };

        let sanitized_history = sanitize_history(history);

        if sanitized_history.is_empty() {
            return "[Error] Chat history is empty.".to_string();
        }

        let mut api_messages = vec![json!({
            "role": "system",
            "content": system_prompt
        })];

        for message in sanitized_history {
            api_messages.push(json!({
                "role": message.role,
                "content": message.content
            }));
        }

        let payload = json!({
            "model": hf_model,
            "messages": api_messages,
            "max_tokens": 300,
            "temperature": 0.35
        });

        let response = match client
            .post("https://router.huggingface.co/v1/chat/completions")
            .bearer_auth(hf_token)
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => return format!("[Error] Failed to connect to AI model: {}", error),
        };

        let status = response.status();

        let response_json: Value = match response.json().await {
            Ok(value) => value,
            Err(error) => return format!("[Error] Failed to parse AI response: {}", error),
        };

        if !status.is_success() {
            return extract_error_message(&response_json).unwrap_or_else(|| {
                format!(
                    "[Error] Hugging Face Router returned {}: {}",
                    status, response_json
                )
            });
        }

        extract_chat_message(&response_json)
            .unwrap_or_else(|| "[Error] Unexpected response format from AI model.".to_string())
    }
}

fn sanitize_history(history: &[ChatMessage]) -> Vec<ChatMessage> {
    history
        .iter()
        .filter_map(|message| {
            let role = message.role.trim();
            let content = message.content.trim();

            if content.is_empty() || !matches!(role, "user" | "assistant") {
                return None;
            }

            Some(ChatMessage {
                role: role.to_string(),
                content: content.to_string(),
            })
        })
        .rev()
        .take(12)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn extract_chat_message(value: &Value) -> Option<String> {
    value
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(Value::as_str)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

fn extract_error_message(value: &Value) -> Option<String> {
    value
        .get("error")
        .and_then(Value::as_str)
        .map(|message| format!("[Error] Hugging Face Router error: {}", message))
        .or_else(|| {
            value
                .get("message")
                .and_then(Value::as_str)
                .map(|message| format!("[Error] Hugging Face Router error: {}", message))
        })
}
