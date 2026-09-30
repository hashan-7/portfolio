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
        return out_of_scope_reply(profile);
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
        match find_requested_project(&question, profile, recent_context) {
            ProjectLookup::Found(index) => return project_detail_reply(profile, index),
            ProjectLookup::OutOfRange { requested, total } => {
                if total == 0 {
                    return "No public chatbot projects are available in the portfolio yet."
                        .to_string();
                }

                return format!(
                    "Project number {requested} is not available. Choose a number from 1 to {total}."
                );
            }
            ProjectLookup::NotSpecified if asks_more_details(&question) => {
                return "Ask a project number or title so I can show the confirmed details from the portfolio data.".to_string();
            }
            ProjectLookup::NotSpecified => {}
        }
    }

    if let Some(index) = find_project_by_title(&question, profile) {
        return project_detail_reply(profile, index);
    }

    if asks_links(&question) {
        return contact_reply(profile.social_links.as_ref());
    }

    out_of_scope_reply(profile)
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .replace(['\n', '\r'], " ")
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
        .filter(|project| project.public_display && project.chatbot_visible)
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

fn out_of_scope_reply(profile: &FullProfile) -> String {
    format!(
        "I can answer only from {}'s portfolio data. Please ask about the profile, projects, skills, certificates, education, focus areas, or contact links.",
        display_name(profile)
    )
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
        .find(|part| !part.is_empty())
        .map(|part| part.parse::<usize>().unwrap_or(usize::MAX))
}

enum ProjectLookup {
    Found(usize),
    OutOfRange { requested: usize, total: usize },
    NotSpecified,
}

fn find_requested_project(
    question: &str,
    profile: &FullProfile,
    recent_context: &[String],
) -> ProjectLookup {
    let projects = visible_projects(profile);

    if let Some(index) = find_project_by_title(question, profile) {
        return ProjectLookup::Found(index);
    }

    if let Some(number) = extract_number(question) {
        if number > 0 && number <= projects.len() {
            return ProjectLookup::Found(number - 1);
        }

        return ProjectLookup::OutOfRange {
            requested: number,
            total: projects.len(),
        };
    }

    if !asks_more_details(question) {
        return ProjectLookup::NotSpecified;
    }

    for context in recent_context {
        let normalized_context = normalize(context);
        let Some(user_message) = normalized_context.strip_prefix("user: ") else {
            continue;
        };

        if let Some(index) = find_project_by_title(user_message, profile) {
            return ProjectLookup::Found(index);
        }

        let is_explicit_selection = is_number_only(user_message)
            || (contains_any(user_message, &["project", "project-"])
                && extract_number(user_message).is_some());

        if is_explicit_selection {
            let number = extract_number(user_message).expect("selection contains a number");
            if number > 0 && number <= projects.len() {
                return ProjectLookup::Found(number - 1);
            }
        }
    }

    ProjectLookup::NotSpecified
}

fn find_project_by_title(question: &str, profile: &FullProfile) -> Option<usize> {
    visible_projects(profile).iter().position(|project| {
        project
            .title
            .as_ref()
            .map(|title| normalize(title))
            .filter(|title| !title.is_empty())
            .map(|title| contains_phrase(question, &title))
            .unwrap_or(false)
    })
}

fn contains_phrase(value: &str, phrase: &str) -> bool {
    value.match_indices(phrase).any(|(start, matched)| {
        let end = start + matched.len();
        let before_is_word = value[..start]
            .chars()
            .next_back()
            .is_some_and(char::is_alphanumeric);
        let after_is_word = value[end..]
            .chars()
            .next()
            .is_some_and(char::is_alphanumeric);

        !before_is_word && !after_is_word
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

    let projects = visible_projects(profile);
    let mut categories = projects
        .iter()
        .filter_map(|project| project.category.as_deref())
        .map(str::trim)
        .filter(|category| !category.is_empty())
        .collect::<Vec<_>>();
    categories.sort_unstable_by_key(|category| category.to_ascii_lowercase());
    categories.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    let category_summary = if categories.is_empty() {
        String::new()
    } else {
        format!(" Listed categories: {}.", categories.join(", "))
    };
    lines.push(format!(
        "3. Project experience: The portfolio includes {} public chatbot-visible projects.{category_summary}",
        projects.len()
    ));

    lines.push(format!(
        "4. Learning proof: The portfolio includes {} certificates.",
        profile.certificates.len()
    ));

    if !profile.education.is_empty() {
        lines.push(format!(
            "5. Education background: The portfolio includes {} education entries.",
            profile.education.len()
        ));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn project(title: &str, public_display: bool, chatbot_visible: bool) -> Project {
        Project {
            title: Some(title.to_string()),
            public_display,
            chatbot_visible,
            ..Project::default()
        }
    }

    fn profile_with_projects(projects: Vec<Project>) -> FullProfile {
        FullProfile {
            display_name: Some("Chamira".to_string()),
            projects,
            ..FullProfile::default()
        }
    }

    #[test]
    fn chatbot_only_uses_public_and_explicitly_visible_projects() {
        let profile = profile_with_projects(vec![
            project("Public", true, true),
            project("Private", false, true),
            project("Bot hidden", true, false),
        ]);

        let list = get_portfolio_reply(&profile, "list projects", &[]);
        assert!(list.contains("Public"));
        assert!(!list.contains("Private"));
        assert!(!list.contains("Bot hidden"));
        assert!(get_portfolio_reply(&profile, "how many projects", &[]).contains("1 project"));
    }

    #[test]
    fn strengths_use_only_listed_project_categories() {
        let mut item = project("Design system", true, true);
        item.category = Some("Design tooling".to_string());
        let profile = profile_with_projects(vec![item]);

        let reply = get_portfolio_reply(&profile, "what are the strengths", &[]);
        assert!(reply.contains("Design tooling"));
        assert!(!reply.contains("backend, AI integration"));
        assert!(!reply.contains("mobile"));
    }

    #[test]
    fn out_of_scope_reply_uses_the_configured_display_name() {
        let profile = profile_with_projects(Vec::new());
        let reply = get_portfolio_reply(&profile, "tell me the weather", &[]);

        assert!(reply.contains("Chamira's portfolio data"));
        assert!(!reply.contains("Chamira Hashan's portfolio data"));
    }

    #[test]
    fn project_details_use_verified_notes_but_never_private_safe_notes() {
        let mut item = project("Verified project", true, true);
        item.internal_chatbot_notes = Some("Verified implementation detail".to_string());
        item.safe_notes = Some("Private owner note".to_string());
        let profile = profile_with_projects(vec![item]);

        let reply = get_portfolio_reply(&profile, "project 1", &[]);
        assert!(reply.contains("Verified implementation detail"));
        assert!(!reply.contains("Private owner note"));
    }

    #[test]
    fn contact_reply_never_exposes_instagram() {
        let profile = FullProfile {
            social_links: Some(SocialLinks {
                github: Some("https://github.com/example".to_string()),
                instagram: Some("https://instagram.com/private".to_string()),
                ..SocialLinks::default()
            }),
            ..FullProfile::default()
        };

        let reply = get_portfolio_reply(&profile, "contact details", &[]);
        assert!(reply.contains("GitHub"));
        assert!(!reply.to_lowercase().contains("instagram"));
        assert!(!reply.contains("instagram.com/private"));
    }

    #[test]
    fn empty_project_titles_never_match() {
        let profile = profile_with_projects(vec![
            project("", true, true),
            project("Rust API", true, true),
        ]);

        let reply = get_portfolio_reply(&profile, "something unrelated", &[]);
        assert!(reply.starts_with("I can answer only"));
    }

    #[test]
    fn follow_up_ignores_assistant_project_numbers() {
        let profile = profile_with_projects(vec![
            project("First", true, true),
            project("Second", true, true),
        ]);
        let context = vec![
            "user: yes".to_string(),
            "assistant: 1. First\n2. Second".to_string(),
            "user: list projects".to_string(),
        ];

        let reply = get_portfolio_reply(&profile, "yes", &context);
        assert!(reply.starts_with("Ask a project number or title"));
    }

    #[test]
    fn follow_up_uses_only_an_explicit_prior_user_selection() {
        let profile = profile_with_projects(vec![
            project("First", true, true),
            project("Second", true, true),
        ]);
        let context = vec![
            "user: more".to_string(),
            "assistant: Would you like more?".to_string(),
            "user: project 2".to_string(),
        ];

        let reply = get_portfolio_reply(&profile, "more", &context);
        assert!(reply.contains("Title: Second"));
    }

    #[test]
    fn explicit_out_of_range_project_number_is_actionable() {
        let profile = profile_with_projects(vec![project("Only", true, true)]);

        let reply = get_portfolio_reply(&profile, "project 9", &[]);
        assert_eq!(
            reply,
            "Project number 9 is not available. Choose a number from 1 to 1."
        );
    }

    #[test]
    fn title_matching_uses_word_boundaries() {
        let profile = profile_with_projects(vec![project("AI", true, true)]);

        let unrelated = get_portfolio_reply(&profile, "email", &[]);
        assert!(!unrelated.contains("Title: AI"));
        let relevant = get_portfolio_reply(&profile, "tell me about AI", &[]);
        assert!(relevant.contains("Title: AI"));
    }
}
