use chatbot_ml::chatbot::client::ChatMessage;

use crate::profile::{Certificate, ChatScope, Education, FullProfile, Project};

pub fn get_portfolio_reply(
    profile: &FullProfile,
    scope: ChatScope,
    history: &[ChatMessage],
) -> Option<String> {
    let latest_user_message = latest_user_message(history)?;
    let normalized = normalize(latest_user_message);
    let tokens = tokenize(&normalized);

    if !matches!(scope, ChatScope::All)
        && !is_message_allowed_in_scope(scope, &normalized, &tokens, history)
    {
        return Some(section_mode_reply(scope));
    }

    if is_greeting(&tokens) {
        return Some(greeting_reply(profile));
    }

    if is_identity_question(&normalized, &tokens) {
        return Some(identity_reply(profile));
    }

    if is_profile_summary_request(&normalized, &tokens) {
        return Some(profile_summary_reply(profile));
    }

    if is_strength_summary_request(&normalized, &tokens) {
        return Some(strength_summary_reply(profile));
    }

    if is_skill_request(&normalized, &tokens)
        && !is_project_specific_question(&normalized, &tokens, history)
    {
        return Some(skills_reply(profile, &normalized, &tokens));
    }

    if is_certificate_request(&normalized, &tokens) {
        return Some(certificates_reply(profile, &normalized, &tokens));
    }

    if is_education_request(&normalized, &tokens) {
        return Some(education_reply(profile, &normalized, &tokens));
    }

    if is_contact_request(&normalized, &tokens) {
        return Some(contact_reply(profile));
    }

    if is_focus_request(&normalized, &tokens) {
        return Some(focus_reply(profile));
    }

    if is_project_request(&normalized, &tokens, history) {
        return handle_project_request(profile, history, &normalized, &tokens);
    }

    None
}

fn latest_user_message(history: &[ChatMessage]) -> Option<&str> {
    history
        .iter()
        .rev()
        .find(|message| message.role.trim() == "user")
        .map(|message| message.content.trim())
        .filter(|content| !content.is_empty())
}

fn normalize(input: &str) -> String {
    input
        .to_lowercase()
        .replace('’', "'")
        .replace('“', "\"")
        .replace('”', "\"")
        .trim()
        .to_string()
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

fn clean(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

fn owner_name(profile: &FullProfile) -> String {
    clean(&profile.display_name)
        .or_else(|| clean(&profile.name))
        .unwrap_or_else(|| "the portfolio owner".to_string())
}

fn visible_projects(profile: &FullProfile) -> Vec<&Project> {
    profile
        .projects
        .iter()
        .filter(|project| project.chatbot_visible)
        .collect()
}

fn is_message_allowed_in_scope(
    scope: ChatScope,
    input: &str,
    tokens: &[String],
    history: &[ChatMessage],
) -> bool {
    match scope {
        ChatScope::All => true,
        ChatScope::Profile => {
            is_identity_question(input, tokens)
                || is_profile_summary_request(input, tokens)
                || is_strength_summary_request(input, tokens)
                || has_any_token(
                    tokens,
                    &["profile", "bio", "role", "name", "about", "location"],
                )
        }
        ChatScope::Skills => is_skill_request(input, tokens),
        ChatScope::Projects => is_project_request(input, tokens, history),
        ChatScope::Certificates => is_certificate_request(input, tokens),
        ChatScope::Education => is_education_request(input, tokens),
        ChatScope::Contact => is_contact_request(input, tokens),
        ChatScope::Focus => is_focus_request(input, tokens),
    }
}

fn section_mode_reply(scope: ChatScope) -> String {
    format!(
        "I am currently focused on {}. Please exit section mode or use the relevant section to ask about that topic.",
        scope.as_display_label()
    )
}

fn is_greeting(tokens: &[String]) -> bool {
    tokens.len() <= 3 && has_any_token(tokens, &["hi", "hello", "hey", "welcome"])
}

fn greeting_reply(profile: &FullProfile) -> String {
    format!(
        "Hello. I am the portfolio assistant for {}. How can I help you today?",
        owner_name(profile)
    )
}

fn is_identity_question(input: &str, tokens: &[String]) -> bool {
    has_phrase(
        input,
        &[
            "who are you",
            "what are you",
            "your name",
            "are you hashan",
            "are you chamira",
            "who is hashan",
            "who is chamira",
            "about hashan",
            "about chamira",
        ],
    ) || (has_token(tokens, "who") && has_any_token(tokens, &["hashan", "chamira"]))
}

fn identity_reply(profile: &FullProfile) -> String {
    let name = owner_name(profile);

    if let Some(role) = clean(&profile.role) {
        return format!(
            "I am the portfolio assistant for {}. According to the portfolio information, {} is {}.",
            name, name, role
        );
    }

    format!(
        "I am the portfolio assistant for {}. I answer using the provided portfolio information.",
        name
    )
}

fn is_profile_summary_request(input: &str, tokens: &[String]) -> bool {
    has_phrase(
        input,
        &[
            "overall profile",
            "professional profile",
            "explain his profile",
            "explain the profile",
            "about his profile",
            "what kind of developer",
            "tell me about chamira professionally",
            "tell me about hashan professionally",
        ],
    ) || (has_token(tokens, "profile")
        && has_any_token(tokens, &["overall", "professional", "explain", "summary"]))
}

fn is_strength_summary_request(input: &str, tokens: &[String]) -> bool {
    has_phrase(
        input,
        &[
            "portfolio strengths",
            "his strengths",
            "main strengths",
            "summarize his strengths",
            "summarize portfolio strengths",
        ],
    ) || has_any_token(tokens, &["strength", "strengths"])
}

fn profile_summary_reply(profile: &FullProfile) -> String {
    let name = owner_name(profile);
    let mut lines = Vec::new();

    if let Some(role) = clean(&profile.role) {
        lines.push(format!(
            "{} is presented in the portfolio as {}.",
            name, role
        ));
    } else {
        lines.push(format!("{} is the portfolio owner.", name));
    }

    if let Some(tagline) = clean(&profile.tagline) {
        lines.push(format!("Tagline: {}", tagline));
    }

    if let Some(bio) = clean(&profile.bio) {
        lines.push(format!("Profile summary: {}", bio));
    }

    if !profile.focus_areas.is_empty() {
        lines.push(format!(
            "Main focus areas: {}",
            profile.focus_areas.join(", ")
        ));
    }

    if !profile.skills.is_empty() {
        lines.push(format!("Key listed skills: {}", profile.skills.join(", ")));
    }

    let project_count = profile
        .projects
        .iter()
        .filter(|project| project.chatbot_visible)
        .count();

    if project_count > 0 {
        lines.push(format!(
            "The portfolio includes {} listed projects.",
            project_count
        ));
    }

    if !profile.certificates.is_empty() {
        lines.push(format!(
            "The portfolio includes {} certificates.",
            profile.certificates.len()
        ));
    }

    if !profile.education.is_empty() {
        lines.push(format!(
            "The portfolio includes {} education entries.",
            profile.education.len()
        ));
    }

    lines.push("This summary is based only on the provided portfolio data.".to_string());

    lines.join("\n")
}

fn strength_summary_reply(profile: &FullProfile) -> String {
    let name = owner_name(profile);
    let mut lines = vec![format!(
        "Based on the portfolio information, {}'s main strengths are:",
        name
    )];

    let mut number = 1;

    if !profile.focus_areas.is_empty() {
        lines.push(format!(
            "{}. Practical focus areas: {}",
            number,
            profile.focus_areas.join(", ")
        ));
        number += 1;
    }

    if !profile.skills.is_empty() {
        lines.push(format!(
            "{}. Technical skill range: {}",
            number,
            profile.skills.join(", ")
        ));
        number += 1;
    }

    let project_count = profile
        .projects
        .iter()
        .filter(|project| project.chatbot_visible)
        .count();

    if project_count > 0 {
        lines.push(format!(
            "{}. Project experience: The portfolio includes {} projects across backend, AI integration, full-stack, mobile, and ML-related work.",
            number, project_count
        ));
        number += 1;
    }

    if !profile.certificates.is_empty() {
        lines.push(format!(
            "{}. Learning proof: The portfolio includes {} certificates.",
            number,
            profile.certificates.len()
        ));
        number += 1;
    }

    if !profile.education.is_empty() {
        lines.push(format!(
            "{}. Software engineering education background is included in the portfolio.",
            number
        ));
    }

    lines.push("These points are based only on the provided portfolio data.".to_string());

    lines.join("\n")
}

fn is_project_request(input: &str, tokens: &[String], history: &[ChatMessage]) -> bool {
    has_any_token(
        tokens,
        &["project", "projects", "app", "apps", "github", "demo"],
    ) || has_phrase(input, &["live demo", "hugging face", "hf link", "git hub"])
        || is_project_followup(input, tokens, history)
}

fn is_project_followup(input: &str, tokens: &[String], history: &[ChatMessage]) -> bool {
    if has_any_token(
        tokens,
        &[
            "profile",
            "strength",
            "strengths",
            "summary",
            "summarize",
            "professional",
            "skill",
            "skills",
            "education",
            "certificate",
            "certificates",
        ],
    ) {
        return false;
    }

    let has_previous_project_context = history.iter().rev().skip(1).take(8).any(|message| {
        let text = normalize(&message.content);
        text.contains("project")
            || text.contains("stitch qa")
            || text.contains("pulseaid")
            || text.contains("hvtm")
    });

    has_previous_project_context
        && (extract_number(input, tokens).is_some()
            || has_any_token(
                tokens,
                &[
                    "first", "second", "third", "last", "only", "ex", "explain", "details",
                    "detail", "link", "github",
                ],
            ))
}

fn is_project_specific_question(input: &str, tokens: &[String], history: &[ChatMessage]) -> bool {
    let has_project_word = has_any_token(tokens, &["project", "projects"]);
    let has_number = extract_number(input, tokens).is_some();
    let asks_link = asks_for_project_link(input, tokens);
    let asks_details = asks_for_project_details(input, tokens);

    has_project_word
        && (has_number || asks_link || asks_details || is_project_followup(input, tokens, history))
}

fn handle_project_request(
    profile: &FullProfile,
    history: &[ChatMessage],
    input: &str,
    tokens: &[String],
) -> Option<String> {
    let projects = visible_projects(profile);

    if projects.is_empty() {
        return Some("Project information is not available in the portfolio data.".to_string());
    }

    let asks_for_link = asks_for_project_link(input, tokens);
    let asks_for_details = asks_for_project_details(input, tokens);
    let asks_for_count = has_any_token(tokens, &["count", "many"]);
    let asks_for_all = has_token(tokens, "all");

    if asks_for_count {
        return Some(format!(
            "There are {} projects listed in {}'s portfolio.",
            projects.len(),
            owner_name(profile)
        ));
    }

    if asks_for_link {
        let project =
            resolve_project_reference(history, &projects, input, tokens).unwrap_or_else(|| {
                projects
                    .first()
                    .copied()
                    .expect("projects should not be empty")
            });

        return Some(project_link_reply(project));
    }

    if asks_for_all && asks_for_details {
        return Some(project_details_list_reply(&projects));
    }

    if let Some(limit) = extract_limit(input, tokens, projects.len()) {
        return Some(project_titles_reply(profile, &projects, Some(limit)));
    }

    if let Some(project) = resolve_project_reference(history, &projects, input, tokens) {
        if asks_for_details || is_direct_project_reference(input, tokens, history) {
            return Some(project_detail_reply(project));
        }
    }

    if asks_for_details {
        return Some(project_titles_reply(profile, &projects, None));
    }

    Some(project_titles_reply(profile, &projects, None))
}

fn is_direct_project_reference(input: &str, tokens: &[String], history: &[ChatMessage]) -> bool {
    let has_number = extract_number(input, tokens).is_some();
    let has_project_word = has_any_token(tokens, &["project", "projects"]);

    let has_previous_project_context = history.iter().rev().skip(1).take(8).any(|message| {
        let text = normalize(&message.content);
        text.contains("project")
            || text.contains("stitch qa")
            || text.contains("pulseaid")
            || text.contains("hvtm")
    });

    has_number && (has_project_word || has_previous_project_context)
}

fn asks_for_project_details(input: &str, tokens: &[String]) -> bool {
    has_any_token(
        tokens,
        &[
            "ex",
            "explain",
            "details",
            "detail",
            "describe",
            "description",
            "about",
            "learn",
        ],
    ) || has_phrase(input, &["tell me about", "what is"])
}

fn asks_for_project_link(input: &str, tokens: &[String]) -> bool {
    has_any_token(tokens, &["link", "links", "github", "demo"])
        || has_phrase(input, &["live demo", "hugging face", "hf link", "git hub"])
}

fn extract_limit(input: &str, tokens: &[String], max: usize) -> Option<usize> {
    if has_token(tokens, "all") {
        return None;
    }

    if has_token(tokens, "only") || has_phrase(input, &["top"]) {
        if let Some(number) = extract_number(input, tokens) {
            return Some(number.min(max).max(1));
        }
    }

    None
}

fn extract_number(input: &str, tokens: &[String]) -> Option<usize> {
    for token in tokens {
        if let Ok(number) = token.parse::<usize>() {
            return Some(number);
        }
    }

    if has_token(tokens, "first") {
        return Some(1);
    }

    if has_token(tokens, "second") {
        return Some(2);
    }

    if has_token(tokens, "third") {
        return Some(3);
    }

    if has_token(tokens, "fourth") {
        return Some(4);
    }

    if has_token(tokens, "fifth") {
        return Some(5);
    }

    if input.trim() == "ex" {
        return Some(1);
    }

    None
}

fn resolve_project_reference<'a>(
    history: &[ChatMessage],
    projects: &'a [&Project],
    input: &str,
    tokens: &[String],
) -> Option<&'a Project> {
    if has_token(tokens, "last") {
        return projects.last().copied();
    }

    if let Some(number) = extract_number(input, tokens) {
        if number > 0 && number <= projects.len() {
            return projects.get(number - 1).copied();
        }
    }

    for project in projects {
        if let Some(title) = clean(&project.title) {
            if input.contains(&title.to_lowercase()) {
                return Some(project);
            }
        }
    }

    for message in history.iter().rev().skip(1).take(8) {
        let text = normalize(&message.content);

        for project in projects {
            if let Some(title) = clean(&project.title) {
                if text.contains(&title.to_lowercase()) {
                    return Some(project);
                }
            }
        }
    }

    None
}

fn project_titles_reply(
    profile: &FullProfile,
    projects: &[&Project],
    limit: Option<usize>,
) -> String {
    let count = limit.unwrap_or(projects.len()).min(projects.len());

    let mut lines = vec![format!(
        "{} has the following projects listed in the portfolio:",
        owner_name(profile)
    )];

    for (index, project) in projects.iter().take(count).enumerate() {
        let title = clean(&project.title).unwrap_or_else(|| format!("Project {}", index + 1));
        lines.push(format!("{}. {}", index + 1, title));
    }

    lines.push("Ask a project number or title if you want details.".to_string());

    lines.join("\n")
}

fn project_details_list_reply(projects: &[&Project]) -> String {
    let mut lines = vec!["Here are the project details available in the portfolio:".to_string()];

    for (index, project) in projects.iter().enumerate() {
        lines.push(String::new());
        lines.push(format!(
            "{}. {}",
            index + 1,
            clean(&project.title).unwrap_or_else(|| "Untitled Project".to_string())
        ));

        if let Some(category) = clean(&project.category) {
            lines.push(format!("Category: {}", category));
        }

        if let Some(description) = clean(&project.short_description) {
            lines.push(format!("Description: {}", description));
        }

        if !project.tech_stack.is_empty() {
            lines.push(format!("Tech stack: {}", project.tech_stack.join(", ")));
        }
    }

    lines.join("\n")
}

fn project_detail_reply(project: &Project) -> String {
    let mut lines = vec![format!(
        "Title: {}",
        clean(&project.title).unwrap_or_else(|| "Untitled Project".to_string())
    )];

    if let Some(category) = clean(&project.category) {
        lines.push(format!("Category: {}", category));
    }

    if let Some(description) = clean(&project.short_description) {
        lines.push(format!("Description: {}", description));
    }

    if !project.tech_stack.is_empty() {
        lines.push(format!("Tech stack: {}", project.tech_stack.join(", ")));
    }

    if let Some(notes) = clean(&project.internal_chatbot_notes) {
        lines.push(format!("Additional details: {}", notes));
    }

    lines.join("\n")
}

fn project_link_reply(project: &Project) -> String {
    let title = clean(&project.title).unwrap_or_else(|| "this project".to_string());
    let mut lines = vec![format!("Links for {}:", title)];
    let mut found = false;

    if let Some(link) = clean(&project.github_link) {
        lines.push(format!("GitHub: {}", link));
        found = true;
    }

    if let Some(link) = clean(&project.hf_link) {
        lines.push(format!("Hugging Face: {}", link));
        found = true;
    }

    if let Some(link) = clean(&project.live_demo_link) {
        lines.push(format!("Live demo: {}", link));
        found = true;
    }

    if !found {
        return format!(
            "The requested link for {} is not available in the portfolio data.",
            title
        );
    }

    lines.join("\n")
}

fn is_skill_request(input: &str, tokens: &[String]) -> bool {
    has_any_token(tokens, &["skill", "skills", "stack"])
        || has_phrase(input, &["tech stack", "technologies"])
}

fn skills_reply(profile: &FullProfile, input: &str, tokens: &[String]) -> String {
    if profile.skills.is_empty() {
        return "Skill information is not available in the portfolio data.".to_string();
    }

    if has_any_token(tokens, &["count", "many"]) {
        return format!(
            "There are {} skills listed in the portfolio.",
            profile.skills.len()
        );
    }

    if has_token(tokens, "last") {
        if let Some(last_skill) = profile.skills.last() {
            return format!("The last skill listed in the portfolio is {}.", last_skill);
        }
    }

    if has_phrase(input, &["based on the project", "based on projects"]) {
        return format!(
            "The listed skills are supported by the portfolio projects. The main listed skills are: {}.",
            profile.skills.join(", ")
        );
    }

    let mut lines = vec!["The skills listed in the portfolio are:".to_string()];

    for (index, skill) in profile.skills.iter().enumerate() {
        lines.push(format!("{}. {}", index + 1, skill));
    }

    lines.join("\n")
}

fn is_certificate_request(input: &str, tokens: &[String]) -> bool {
    has_any_token(
        tokens,
        &["certificate", "certificates", "certifi", "cert", "certs"],
    ) || input.contains("certificate")
}

fn certificates_reply(profile: &FullProfile, input: &str, tokens: &[String]) -> String {
    if profile.certificates.is_empty() {
        return "Certificate information is not available in the portfolio data.".to_string();
    }

    if has_any_token(tokens, &["count", "many"]) {
        return format!(
            "There are {} certificates listed in the portfolio.",
            profile.certificates.len()
        );
    }

    let asks_for_link =
        has_any_token(tokens, &["link", "links"]) || has_phrase(input, &["last lins", "last link"]);

    if asks_for_link {
        let certificate = resolve_certificate_reference(&profile.certificates, input, tokens);
        return certificate_link_reply(certificate);
    }

    let mut lines = vec!["The certificates listed in the portfolio are:".to_string()];

    for (index, certificate) in profile.certificates.iter().enumerate() {
        let name = clean(&certificate.name).unwrap_or_else(|| format!("Certificate {}", index + 1));
        let date = clean(&certificate.date).or_else(|| clean(&certificate.year));

        if let Some(date) = date {
            lines.push(format!("{}. {} ({})", index + 1, name, date));
        } else {
            lines.push(format!("{}. {}", index + 1, name));
        }
    }

    lines.join("\n")
}

fn resolve_certificate_reference<'a>(
    certificates: &'a [Certificate],
    input: &str,
    tokens: &[String],
) -> Option<&'a Certificate> {
    if certificates.is_empty() {
        return None;
    }

    if has_token(tokens, "last") {
        return certificates.last();
    }

    if let Some(number) = extract_number(input, tokens) {
        if number > 0 && number <= certificates.len() {
            return certificates.get(number - 1);
        }
    }

    for certificate in certificates {
        if let Some(name) = clean(&certificate.name) {
            if input.contains(&name.to_lowercase()) {
                return Some(certificate);
            }
        }
    }

    certificates.last()
}

fn certificate_link_reply(certificate: Option<&Certificate>) -> String {
    let Some(certificate) = certificate else {
        return "The requested certificate link is not available in the portfolio data."
            .to_string();
    };

    let name = clean(&certificate.name).unwrap_or_else(|| "the requested certificate".to_string());

    if let Some(link) = clean(&certificate.link) {
        return format!("The link for {} is:\n{}", name, link);
    }

    format!(
        "The link for {} is not available in the portfolio data.",
        name
    )
}

fn is_education_request(input: &str, tokens: &[String]) -> bool {
    has_any_token(
        tokens,
        &[
            "education",
            "educations",
            "edcation",
            "school",
            "campus",
            "nibm",
            "gpa",
            "degree",
            "diploma",
            "hnd",
            "institute",
            "institution",
            "instituted",
            "qualifier",
        ],
    ) || input.contains("higher national diploma")
}

fn education_reply(profile: &FullProfile, input: &str, tokens: &[String]) -> String {
    if profile.education.is_empty() {
        return "Education information is not available in the portfolio data.".to_string();
    }

    if has_token(tokens, "gpa") {
        return gpa_reply(&profile.education);
    }

    if has_phrase(input, &["degree or not"])
        || (has_token(tokens, "degree") && has_token(tokens, "not"))
    {
        return degree_status_reply(&profile.education);
    }

    if has_any_token(tokens, &["latest", "last"]) {
        return latest_education_reply(&profile.education);
    }

    if has_any_token(tokens, &["count", "many"]) {
        return education_count_reply(&profile.education, tokens);
    }

    if has_token(tokens, "nibm") || has_any_token(tokens, &["diploma", "hnd", "degree"]) {
        return nibm_reply(&profile.education);
    }

    if has_token(tokens, "school") {
        return school_reply(&profile.education);
    }

    if has_any_token(
        tokens,
        &[
            "campus",
            "institute",
            "institution",
            "instituted",
            "qualifier",
        ],
    ) {
        return education_centers_reply(&profile.education);
    }

    let mut lines = vec!["Here are the education details listed in the portfolio:".to_string()];

    for (index, item) in profile.education.iter().enumerate() {
        lines.push(format!("{}. {}", index + 1, education_line(item)));
    }

    lines.join("\n")
}

fn education_line(item: &Education) -> String {
    let mut parts = Vec::new();

    if let Some(institution) = clean(&item.institution) {
        parts.push(format!("Institution: {}", institution));
    }

    if let Some(degree) = clean(&item.degree) {
        parts.push(format!("Program: {}", degree));
    }

    if let Some(duration) = clean(&item.duration).or_else(|| clean(&item.year)) {
        parts.push(format!("Duration: {}", duration));
    }

    if let Some(grade) = clean(&item.grade) {
        parts.push(format!("Grade: {}", grade));
    }

    if let Some(status) = clean(&item.status) {
        parts.push(format!("Status: {}", status));
    }

    parts.join(", ")
}

fn latest_education_reply(education: &[Education]) -> String {
    if let Some(item) = education.last() {
        return format!("The latest education entry is:\n{}", education_line(item));
    }

    "Latest education information is not available in the portfolio data.".to_string()
}

fn gpa_reply(education: &[Education]) -> String {
    for item in education {
        if let Some(grade) = clean(&item.grade) {
            if grade.to_lowercase().contains("gpa") || grade.contains('/') {
                let degree =
                    clean(&item.degree).unwrap_or_else(|| "the listed program".to_string());
                return format!("The GPA listed for {} is {}.", degree, grade);
            }
        }
    }

    "GPA information is not available in the portfolio data.".to_string()
}

fn degree_status_reply(education: &[Education]) -> String {
    let has_degree = education.iter().any(|item| {
        clean(&item.degree)
            .map(|degree| degree.to_lowercase().contains("degree"))
            .unwrap_or(false)
    });

    if has_degree {
        return "The portfolio includes Software Engineering education entries, but it does not mention a completed bachelor's degree. It lists a Diploma in Software Engineering and a Higher National Diploma in Software Engineering with academic work completed.".to_string();
    }

    "The portfolio does not mention a completed degree. It lists school education, a Diploma in Software Engineering, and a Higher National Diploma in Software Engineering.".to_string()
}

fn education_count_reply(education: &[Education], tokens: &[String]) -> String {
    if has_any_token(
        tokens,
        &[
            "center",
            "centers",
            "institute",
            "institution",
            "institutions",
        ],
    ) {
        let centers = unique_education_centers(education);
        return format!(
            "There are {} different education institutions listed: {}.",
            centers.len(),
            centers.join(", ")
        );
    }

    format!("There are {} education entries listed.", education.len())
}

fn education_centers_reply(education: &[Education]) -> String {
    let centers = unique_education_centers(education);

    if centers.is_empty() {
        return "Education institution information is not available in the portfolio data."
            .to_string();
    }

    format!(
        "The education institutions listed are: {}.",
        centers.join(", ")
    )
}

fn unique_education_centers(education: &[Education]) -> Vec<String> {
    let mut centers = Vec::new();

    for item in education {
        if let Some(institution) = clean(&item.institution) {
            if !centers.iter().any(|center: &String| center == &institution) {
                centers.push(institution);
            }
        }
    }

    centers
}

fn nibm_reply(education: &[Education]) -> String {
    let nibm_items = education
        .iter()
        .filter(|item| {
            clean(&item.institution)
                .map(|institution| institution.to_lowercase().contains("nibm"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    if nibm_items.is_empty() {
        return "NIBM education information is not available in the portfolio data.".to_string();
    }

    let mut lines =
        vec!["According to the portfolio, the NIBM qualifications listed are:".to_string()];

    for (index, item) in nibm_items.iter().enumerate() {
        lines.push(format!("{}. {}", index + 1, education_line(item)));
    }

    lines.join("\n")
}

fn school_reply(education: &[Education]) -> String {
    for item in education {
        let degree = clean(&item.degree).unwrap_or_default().to_lowercase();

        if degree.contains("school") {
            if let Some(institution) = clean(&item.institution) {
                return format!("The school listed in the portfolio is {}.", institution);
            }
        }
    }

    "School information is not available in the portfolio data.".to_string()
}

fn is_contact_request(input: &str, tokens: &[String]) -> bool {
    has_any_token(
        tokens,
        &[
            "contact",
            "email",
            "phone",
            "linkedin",
            "github",
            "kaggle",
            "resume",
            "instagram",
            "website",
        ],
    ) || has_phrase(
        input,
        &["how to contact", "contact hashan", "contact chamira"],
    )
}

fn contact_reply(profile: &FullProfile) -> String {
    let Some(links) = &profile.social_links else {
        return "Contact information is not available in the portfolio data.".to_string();
    };

    let mut lines = vec!["The available contact and profile links are:".to_string()];
    push_link(&mut lines, "Email", &links.email);
    push_link(&mut lines, "Phone", &links.phone);
    push_link(&mut lines, "GitHub", &links.github);
    push_link(&mut lines, "LinkedIn", &links.linkedin);
    push_link(&mut lines, "Hugging Face", &links.huggingface);
    push_link(&mut lines, "Kaggle", &links.kaggle);
    push_link(&mut lines, "Resume", &links.resume);
    push_link(&mut lines, "Instagram", &links.instagram);
    push_link(&mut lines, "Website", &links.website);

    if lines.len() == 1 {
        return "Contact information is not available in the portfolio data.".to_string();
    }

    lines.join("\n")
}

fn push_link(lines: &mut Vec<String>, label: &str, value: &Option<String>) {
    if let Some(value) = clean(value) {
        lines.push(format!("{}: {}", label, value));
    }
}

fn is_focus_request(input: &str, tokens: &[String]) -> bool {
    has_any_token(tokens, &["focus", "areas", "specialization", "specialized"])
        || input.contains("what does he build")
}

fn focus_reply(profile: &FullProfile) -> String {
    if profile.focus_areas.is_empty() {
        return "Focus area information is not available in the portfolio data.".to_string();
    }

    let mut lines = vec!["The focus areas listed in the portfolio are:".to_string()];

    for (index, area) in profile.focus_areas.iter().enumerate() {
        lines.push(format!("{}. {}", index + 1, area));
    }

    lines.join("\n")
}
