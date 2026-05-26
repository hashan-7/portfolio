use crate::profile::{Certificate, Education, FullProfile, Project, SocialLinks};

pub fn get_portfolio_reply(
    profile: &FullProfile,
    latest_user_message: &str,
    recent_context: &[String],
) -> String {
    let question = normalize(latest_user_message);

    if question.is_empty() {
        return greeting(profile);
    }

    if is_greeting(&question) {
        return greeting(profile);
    }

    if is_out_of_scope_sensitive(&question) {
        return out_of_scope_reply();
    }

    if asks_contact(&question) {
        return contact_reply(profile.social_links.as_ref());
    }

    if asks_profile_summary(&question) {
        return profile_summary_reply(profile);
    }

    if asks_strengths(&question) {
        return strengths_reply(profile);
    }

    if asks_focus(&question) {
        return focus_reply(profile);
    }

    if asks_skill_count(&question) {
        return format!(
            "There are {} skills listed in the portfolio.",
            profile.skills.len()
        );
    }

    if asks_last_skill(&question) {
        return profile
            .skills
            .last()
            .map(|skill| format!("The last listed skill is {}.", skill))
            .unwrap_or_else(|| "No skills are listed in the portfolio yet.".to_string());
    }

    if asks_skills(&question) {
        return skills_reply(&profile.skills);
    }

    if asks_certificate_count(&question) {
        return format!(
            "There are {} certificates listed in the portfolio.",
            profile.certificates.len()
        );
    }

    if asks_certificates(&question) {
        return certificates_reply(&profile.certificates);
    }

    if asks_education(&question) {
        return education_reply(&profile.education);
    }

    if asks_project_count(&question) {
        return format!(
            "There are {} projects listed in {}'s portfolio.",
            visible_projects(profile).len(),
            display_name(profile)
        );
    }

    if asks_projects_list(&question) {
        return project_list_reply(profile);
    }

    if asks_project_details(&question) || is_number_only(&question) || asks_more_details(&question)
    {
        if let Some(index) = find_requested_project_index(&question, profile, recent_context) {
            return project_detail_reply(profile, index);
        }

        if asks_more_details(&question) {
            return "Ask a project number or title so I can show the confirmed details from the portfolio data.".to_string();
        }
    }

    if let Some(index) = find_project_by_title(&question, profile) {
        return project_detail_reply(profile, index);
    }

    if asks_links(&question) {
        return contact_reply(profile.social_links.as_ref());
    }

    out_of_scope_reply()
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .replace('\n', " ")
        .replace('\r', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn display_name(profile: &FullProfile) -> String {
    profile
        .display_name
        .clone()
        .or_else(|| profile.name.clone())
        .unwrap_or_else(|| "Chamira Hashan".to_string())
}

fn visible_projects(profile: &FullProfile) -> Vec<&Project> {
    profile
        .projects
        .iter()
        .filter(|project| project.chatbot_visible)
        .collect()
}

fn contains_any(question: &str, words: &[&str]) -> bool {
    words.iter().any(|word| question.contains(word))
}

fn is_greeting(question: &str) -> bool {
    matches!(
        question,
        "hi" | "hello" | "hey" | "hrllo" | "helo" | "hii" | "හායි"
    ) || question.starts_with("hi ")
        || question.starts_with("hello ")
}

fn greeting(profile: &FullProfile) -> String {
    format!(
        "Hello. I am H7 Assistant for {}. You can ask about profile, projects, skills, certificates, education, or contact details.",
        display_name(profile)
    )
}

fn is_out_of_scope_sensitive(question: &str) -> bool {
    contains_any(
        question,
        &[
            "password",
            "token",
            "secret",
            "admin password",
            "admin token",
            "api key",
            "hack",
            "bypass",
            "private note",
            "safe_notes",
            "chatbot_rules",
        ],
    )
}

fn out_of_scope_reply() -> String {
    "I can answer only from Chamira Hashan's portfolio data. Please ask about his profile, projects, skills, certificates, education, focus areas, or contact links.".to_string()
}

fn asks_contact(question: &str) -> bool {
    contains_any(
        question,
        &[
            "contact",
            "email",
            "phone",
            "github",
            "linkedin",
            "kaggle",
            "hugging face",
            "huggingface",
            "resume",
            "cv",
            "instagram",
            "website",
            "social",
        ],
    )
}

fn asks_links(question: &str) -> bool {
    contains_any(question, &["link", "links", "url"])
}

fn asks_profile_summary(question: &str) -> bool {
    contains_any(
        question,
        &[
            "overall profile",
            "professional way",
            "profile summary",
            "about him",
            "about chamira",
            "who is",
            "explain his profile",
            "summary of profile",
        ],
    )
}

fn asks_strengths(question: &str) -> bool {
    contains_any(
        question,
        &[
            "strength",
            "strengths",
            "strong",
            "good",
            "portfolio strengths",
            "summarize his portfolio strengths",
        ],
    )
}

fn asks_focus(question: &str) -> bool {
    contains_any(question, &["focus", "focus areas", "interested"])
}

fn asks_skills(question: &str) -> bool {
    contains_any(question, &["skill", "skills", "tech stack", "technologies"])
}

fn asks_skill_count(question: &str) -> bool {
    contains_any(
        question,
        &["skill count", "how many skills", "number of skills"],
    )
}

fn asks_last_skill(question: &str) -> bool {
    contains_any(question, &["last skill", "final skill"])
}

fn asks_certificates(question: &str) -> bool {
    contains_any(
        question,
        &[
            "certificate",
            "certificates",
            "certification",
            "certifications",
            "verified",
        ],
    )
}

fn asks_certificate_count(question: &str) -> bool {
    contains_any(
        question,
        &[
            "certificate count",
            "certificates count",
            "how many certificates",
            "number of certificates",
        ],
    )
}

fn asks_education(question: &str) -> bool {
    contains_any(
        question,
        &[
            "education",
            "educations",
            "school",
            "college",
            "nibm",
            "diploma",
            "hnd",
            "degree",
            "academic",
            "institute",
            "institution",
            "instatued",
        ],
    )
}

fn asks_projects_list(question: &str) -> bool {
    contains_any(question, &["project", "projects"]) && !asks_project_details(question)
}

fn asks_project_count(question: &str) -> bool {
    contains_any(
        question,
        &[
            "project count",
            "projects count",
            "how many projects",
            "number of projects",
        ],
    )
}

fn asks_project_details(question: &str) -> bool {
    contains_any(
        question,
        &[
            "project ",
            "project-",
            "fully explain",
            "full explain",
            "explain project",
            "project details",
            "details about project",
        ],
    )
}

fn asks_more_details(question: &str) -> bool {
    matches!(question, "more" | "more details" | "yes" | "yeah" | "yep")
        || contains_any(question, &["more details", "tell me more", "full details"])
}

fn is_number_only(question: &str) -> bool {
    question.parse::<usize>().is_ok()
}

fn extract_number(question: &str) -> Option<usize> {
    question
        .split(|character: char| !character.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .find_map(|part| part.parse::<usize>().ok())
}

fn find_requested_project_index(
    question: &str,
    profile: &FullProfile,
    recent_context: &[String],
) -> Option<usize> {
    let projects = visible_projects(profile);

    if projects.is_empty() {
        return None;
    }

    if let Some(number) = extract_number(question) {
        if number > 0 && number <= projects.len() {
            return Some(number - 1);
        }
    }

    if let Some(index) = find_project_by_title(question, profile) {
        return Some(index);
    }

    for context in recent_context {
        let normalized_context = normalize(context);

        if let Some(number) = extract_number(&normalized_context) {
            if normalized_context.contains("project") && number > 0 && number <= projects.len() {
                return Some(number - 1);
            }
        }

        if let Some(index) = find_project_by_title(&normalized_context, profile) {
            return Some(index);
        }
    }

    None
}

fn find_project_by_title(question: &str, profile: &FullProfile) -> Option<usize> {
    visible_projects(profile).iter().position(|project| {
        project
            .title
            .as_ref()
            .map(|title| question.contains(&normalize(title)))
            .unwrap_or(false)
    })
}

fn project_list_reply(profile: &FullProfile) -> String {
    let projects = visible_projects(profile);

    if projects.is_empty() {
        return "No projects are listed in the portfolio yet.".to_string();
    }

    let mut lines = vec![format!(
        "{} has the following projects listed in the portfolio:",
        display_name(profile)
    )];

    for (index, project) in projects.iter().enumerate() {
        lines.push(format!(
            "{}. {}",
            index + 1,
            project.title.as_deref().unwrap_or("Untitled Project")
        ));
    }

    lines.push("Ask a project number or title if you want details.".to_string());
    lines.join("\n")
}

fn project_detail_reply(profile: &FullProfile, index: usize) -> String {
    let projects = visible_projects(profile);

    let Some(project) = projects.get(index) else {
        return "That project number is not available in the portfolio data.".to_string();
    };

    let mut lines = Vec::new();

    lines.push(format!(
        "Title: {}",
        project.title.as_deref().unwrap_or("Untitled Project")
    ));

    if let Some(category) = project
        .category
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        lines.push(format!("Category: {}", category));
    }

    if let Some(description) = project
        .short_description
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        lines.push(format!("Description: {}", description));
    }

    if !project.tech_stack.is_empty() {
        lines.push(format!("Tech stack: {}", project.tech_stack.join(", ")));
    }

    if let Some(notes) = project
        .internal_chatbot_notes
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        lines.push(format!("Additional details: {}", notes));
    }

    let mut links = Vec::new();

    if let Some(link) = project
        .github_link
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        links.push(format!("GitHub: {}", link));
    }

    if let Some(link) = project.hf_link.as_deref().filter(|value| !value.is_empty()) {
        links.push(format!("Hugging Face: {}", link));
    }

    if let Some(link) = project
        .live_demo_link
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        links.push(format!("Live demo: {}", link));
    }

    if !links.is_empty() {
        lines.push(links.join("\n"));
    }

    lines.join("\n")
}

fn skills_reply(skills: &[String]) -> String {
    if skills.is_empty() {
        return "No skills are listed in the portfolio yet.".to_string();
    }

    let mut lines = vec!["The skills listed in the portfolio are:".to_string()];

    for (index, skill) in skills.iter().enumerate() {
        lines.push(format!("{}. {}", index + 1, skill));
    }

    lines.join("\n")
}

fn certificates_reply(certificates: &[Certificate]) -> String {
    if certificates.is_empty() {
        return "No certificates are listed in the portfolio yet.".to_string();
    }

    let mut lines = vec!["The certificates listed in the portfolio are:".to_string()];

    for (index, certificate) in certificates.iter().enumerate() {
        let name = certificate
            .name
            .as_deref()
            .unwrap_or("Untitled Certificate");
        let issuer = certificate
            .issuer
            .as_deref()
            .unwrap_or("Issuer not provided");
        let date = certificate
            .date
            .as_deref()
            .or(certificate.year.as_deref())
            .unwrap_or("Date not provided");

        lines.push(format!("{}. {} — {} ({})", index + 1, name, issuer, date));
    }

    lines.join("\n")
}

fn education_reply(education: &[Education]) -> String {
    if education.is_empty() {
        return "No education details are listed in the portfolio yet.".to_string();
    }

    let mut lines = vec!["Here are the education details listed in the portfolio:".to_string()];

    for (index, item) in education.iter().enumerate() {
        let institution = item
            .institution
            .as_deref()
            .unwrap_or("Institution not provided");
        let degree = item.degree.as_deref().unwrap_or("Program not provided");
        let duration = item
            .duration
            .as_deref()
            .or(item.year.as_deref())
            .unwrap_or("Duration not provided");

        let mut detail = format!(
            "{}. Institution: {}, Program: {}, Duration: {}",
            index + 1,
            institution,
            degree,
            duration
        );

        if let Some(grade) = item.grade.as_deref().filter(|value| !value.is_empty()) {
            detail.push_str(&format!(", Grade: {}", grade));
        }

        if let Some(status) = item.status.as_deref().filter(|value| !value.is_empty()) {
            detail.push_str(&format!(", Status: {}", status));
        }

        lines.push(detail);
    }

    lines.join("\n")
}

fn profile_summary_reply(profile: &FullProfile) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "{} is presented in the portfolio as {}.",
        display_name(profile),
        profile
            .role
            .as_deref()
            .unwrap_or("a software engineering portfolio owner")
    ));

    if let Some(tagline) = profile.tagline.as_deref().filter(|value| !value.is_empty()) {
        lines.push(format!("Tagline: {}", tagline));
    }

    if let Some(bio) = profile.bio.as_deref().filter(|value| !value.is_empty()) {
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

    lines.push(format!(
        "The portfolio includes {} listed projects.",
        visible_projects(profile).len()
    ));

    lines.push(format!(
        "The portfolio includes {} certificates.",
        profile.certificates.len()
    ));

    lines.push(format!(
        "The portfolio includes {} education entries.",
        profile.education.len()
    ));

    lines.push("This summary is based only on the provided portfolio data.".to_string());

    lines.join("\n")
}

fn strengths_reply(profile: &FullProfile) -> String {
    let mut lines = vec![format!(
        "Based on the portfolio information, {}'s main strengths are:",
        display_name(profile)
    )];

    if !profile.focus_areas.is_empty() {
        lines.push(format!(
            "1. Practical focus areas: {}",
            profile.focus_areas.join(", ")
        ));
    }

    if !profile.skills.is_empty() {
        lines.push(format!(
            "2. Technical skill range: {}",
            profile.skills.join(", ")
        ));
    }

    lines.push(format!(
        "3. Project experience: The portfolio includes {} projects across backend, AI integration, full-stack, mobile, and ML-related work.",
        visible_projects(profile).len()
    ));

    lines.push(format!(
        "4. Learning proof: The portfolio includes {} certificates.",
        profile.certificates.len()
    ));

    if !profile.education.is_empty() {
        lines.push(
            "5. Software engineering education background is included in the portfolio."
                .to_string(),
        );
    }

    lines.push("These points are based only on the provided portfolio data.".to_string());

    lines.join("\n")
}

fn focus_reply(profile: &FullProfile) -> String {
    if profile.focus_areas.is_empty() {
        return "No focus areas are listed in the portfolio yet.".to_string();
    }

    format!(
        "The focus areas listed in the portfolio are: {}.",
        profile.focus_areas.join(", ")
    )
}

fn contact_reply(social_links: Option<&SocialLinks>) -> String {
    let Some(links) = social_links else {
        return "No public contact links are listed in the portfolio yet.".to_string();
    };

    let mut lines =
        vec!["The public contact and social links listed in the portfolio are:".to_string()];

    push_optional_line(&mut lines, "GitHub", links.github.as_deref());
    push_optional_line(&mut lines, "LinkedIn", links.linkedin.as_deref());
    push_optional_line(&mut lines, "Email", links.email.as_deref());
    push_optional_line(&mut lines, "Phone", links.phone.as_deref());
    push_optional_line(&mut lines, "Website", links.website.as_deref());
    push_optional_line(&mut lines, "Hugging Face", links.huggingface.as_deref());
    push_optional_line(&mut lines, "Kaggle", links.kaggle.as_deref());
    push_optional_line(&mut lines, "Resume / CV", links.resume.as_deref());
    push_optional_line(&mut lines, "Instagram", links.instagram.as_deref());

    if lines.len() == 1 {
        return "No public contact links are listed in the portfolio yet.".to_string();
    }

    lines.join("\n")
}

fn push_optional_line(lines: &mut Vec<String>, label: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        lines.push(format!("{}: {}", label, value));
    }
}
