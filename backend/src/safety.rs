use chatbot_ml::chatbot::client::ChatMessage;

pub fn get_safety_reply(history: &[ChatMessage]) -> Option<String> {
    let latest_user_message = history
        .iter()
        .rev()
        .find(|message| message.role.trim() == "user")?
        .content
        .trim();

    if latest_user_message.is_empty() {
        return None;
    }

    let normalized = normalize(latest_user_message);
    let tokens = tokenize(&normalized);

    if is_self_harm_message(&normalized, &tokens) {
        return Some(self_harm_reply());
    }

    if is_sexual_violence_message(&normalized, &tokens) {
        return Some(sexual_violence_reply());
    }

    if is_harm_or_illegal_message(&normalized, &tokens) {
        return Some(harm_or_illegal_reply());
    }

    if is_adult_off_topic_message(&tokens) {
        return Some(portfolio_only_reply());
    }

    None
}

fn normalize(input: &str) -> String {
    input
        .to_lowercase()
        .replace('’', "'")
        .replace('“', "\"")
        .replace('”', "\"")
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn has_token(tokens: &[String], value: &str) -> bool {
    tokens.iter().any(|token| token == value)
}

fn has_any_token(tokens: &[String], values: &[&str]) -> bool {
    values.iter().any(|value| has_token(tokens, value))
}

fn has_phrase(input: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| input.contains(phrase))
}

fn is_self_harm_message(input: &str, tokens: &[String]) -> bool {
    has_phrase(
        input,
        &[
            "i want to die",
            "i wanna die",
            "i want die",
            "want to die",
            "kill myself",
            "end my life",
            "take my life",
            "suicide",
            "commit suicide",
            "hurt myself",
            "harm myself",
            "self harm",
        ],
    ) || has_any_token(tokens, &["suicide"])
}

fn is_sexual_violence_message(input: &str, tokens: &[String]) -> bool {
    has_any_token(
        tokens,
        &["rape", "raping", "raped", "reape", "molest", "molesting"],
    ) || has_phrase(
        input,
        &[
            "sexual assault",
            "force sex",
            "force her",
            "force girls",
            "force girl",
            "force women",
            "force woman",
            "against her will",
            "without consent",
        ],
    )
}

fn is_harm_or_illegal_message(input: &str, tokens: &[String]) -> bool {
    let direct_harm = has_phrase(
        input,
        &[
            "how to kill",
            "how can i kill",
            "i want to kill",
            "hurt someone",
            "harm someone",
            "attack someone",
            "make a bomb",
            "build a bomb",
            "hide a body",
        ],
    );

    let direct_illegal = has_phrase(
        input,
        &[
            "how to steal",
            "how to rob",
            "how to scam",
            "hack someone's",
            "hack someone",
            "steal password",
            "bypass password",
            "credit card fraud",
        ],
    );

    direct_harm || direct_illegal || has_any_token(tokens, &["bombmaking"])
}

fn is_adult_off_topic_message(tokens: &[String]) -> bool {
    has_any_token(
        tokens,
        &[
            "sex", "sx", "sexy", "porn", "porno", "nude", "nudes", "naked", "xxx", "horny",
        ],
    )
}

fn self_harm_reply() -> String {
    "I am really sorry you are feeling this way. I cannot help with self-harm, but you do not have to handle this alone. If you are in Sri Lanka, you can call the National Mental Health Helpline at 1926. If you are in immediate danger, call local emergency services or go to the nearest hospital now. Please also contact someone you trust and stay near another person if you can.".to_string()
}

fn sexual_violence_reply() -> String {
    "I cannot help with sexual assault, coercion, or harming anyone. If you feel at risk of hurting someone, step away from the person, contact someone you trust, and seek immediate help from local emergency services or a qualified professional.".to_string()
}

fn harm_or_illegal_reply() -> String {
    "I cannot help with harming people, illegal actions, or instructions that could put someone at risk. I can only help with safe questions about Chamira Hashan's portfolio, projects, skills, education, certificates, and contact links.".to_string()
}

fn portfolio_only_reply() -> String {
    "I can only answer questions about Chamira Hashan's portfolio, projects, skills, education, certificates, and contact links.".to_string()
}
