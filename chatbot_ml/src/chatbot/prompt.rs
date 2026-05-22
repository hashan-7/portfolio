use std::env;

pub fn build_system_prompt() -> String {
    let base_instruction = "You are an AI assistant for a personal portfolio website. Answer in a friendly, professional, and concise way. Use only the provided portfolio information. If the answer is not available in the portfolio information, say that the information is not available.";

    match env::var("PORTFOLIO_PROFILE_JSON") {
        Ok(profile_json) if !profile_json.trim().is_empty() => {
            format!(
                "{}\n\nPortfolio information:\n{}",
                base_instruction,
                profile_json.trim()
            )
        }
        _ => {
            format!(
                "{}\n\nPortfolio information is not currently configured.",
                base_instruction
            )
        }
    }
}