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
    pub async fn get_reply(
        history: &[ChatMessage],
        profile_context_json: &str,
        scope_key: &str,
        scope_label: &str,
    ) -> String {
        let system_prompt = build_system_prompt(profile_context_json, scope_key, scope_label);

        let hf_token = match env::var("HF_API_TOKEN") {
            Ok(token) if !token.trim().is_empty() => token,
            _ => return ai_fallback_unavailable_reply(),
        };

        let hf_model = match env::var("HF_MODEL_ID") {
            Ok(model) if !model.trim().is_empty() => model,
            _ => return ai_fallback_unavailable_reply(),
        };

        if !hf_token.starts_with("hf_") {
            return ai_fallback_unavailable_reply();
        }

        let client = match Client::builder().timeout(Duration::from_secs(60)).build() {
            Ok(client) => client,
            Err(_) => return ai_fallback_unavailable_reply(),
        };

        let sanitized_history = sanitize_history(history);

        if sanitized_history.is_empty() {
            return "Please ask a question about the portfolio.".to_string();
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
            "max_tokens": 900,
            "temperature": 0.2
        });

        let response = match client
            .post("https://router.huggingface.co/v1/chat/completions")
            .bearer_auth(hf_token)
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => response,
            Err(_) => return ai_fallback_unavailable_reply(),
        };

        let status = response.status();

        let response_json: Value = match response.json().await {
            Ok(value) => value,
            Err(_) => return ai_fallback_unavailable_reply(),
        };

        if !status.is_success() {
            if is_credit_or_provider_error(&response_json) {
                return ai_fallback_unavailable_reply();
            }

            return ai_fallback_unavailable_reply();
        }

        extract_chat_message(&response_json)
            .map(clean_model_reply)
            .unwrap_or_else(ai_fallback_unavailable_reply)
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

fn is_credit_or_provider_error(value: &Value) -> bool {
    let text = value.to_string().to_lowercase();

    text.contains("depleted")
        || text.contains("credits")
        || text.contains("monthly included")
        || text.contains("inference providers")
        || text.contains("purchase pre-paid")
}

fn ai_fallback_unavailable_reply() -> String {
    "The AI fallback is temporarily unavailable. I can still answer many direct portfolio questions from the local portfolio data. Please ask about projects, skills, education, certificates, contact links, or a specific project number.".to_string()
}

fn clean_model_reply(text: String) -> String {
    let cleaned_lines = text.lines().map(clean_line).collect::<Vec<_>>().join("\n");

    cleaned_lines
        .replace("**", "")
        .replace("__", "")
        .replace("###", "")
        .replace("##", "")
        .replace('#', "")
        .trim()
        .to_string()
}

fn clean_line(line: &str) -> String {
    let trimmed = line.trim();

    let without_bullet = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("• "))
        .unwrap_or(trimmed);

    without_bullet.trim().to_string()
}
