use super::prompt::build_system_prompt;

pub struct AiClient;

impl AiClient {
    pub async fn get_reply(user_message: &str) -> String {
        let _system_prompt = build_system_prompt();

        format!(
            "[Chatbot ML Layer] Message received: '{}'. The AI model integration is currently in progress.",
            user_message
        )
    }
}