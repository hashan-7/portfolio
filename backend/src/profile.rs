use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fmt};
use utoipa::ToSchema;

pub const MAX_PROFILE_SERIALIZED_BYTES: usize = 1024 * 1024;

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Default)]
pub struct Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Default)]
pub struct SocialLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huggingface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kaggle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Default)]
pub struct Education {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Default)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tech_stack: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hf_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_demo_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_path: Option<String>,
    #[serde(default = "default_true")]
    pub public_display: bool,
    #[serde(default)]
    pub chatbot_visible: bool,
    #[serde(default)]
    pub featured: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_chatbot_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Default)]
pub struct FullProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus_areas: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<Project>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<Certificate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<Education>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<SocialLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatbot_rules: Option<String>,
    #[serde(default, flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct PublicProject {
    pub title: Option<String>,
    pub short_description: Option<String>,
    pub category: Option<String>,
    pub tech_stack: Vec<String>,
    pub github_link: Option<String>,
    pub hf_link: Option<String>,
    pub live_demo_link: Option<String>,
    pub image_path: Option<String>,
    pub image_paths: Vec<String>,
    pub video_path: Option<String>,
    pub featured: bool,
}

#[derive(Serialize, ToSchema)]
pub struct PublicSocialLinks {
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub huggingface: Option<String>,
    pub kaggle: Option<String>,
    pub resume: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct PublicProfile {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub role: Option<String>,
    pub tagline: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub profile_image_path: Option<String>,
    pub skills: Vec<String>,
    pub focus_areas: Vec<String>,
    pub projects: Vec<PublicProject>,
    pub certificates: Vec<Certificate>,
    pub education: Vec<Education>,
    pub social_links: Option<PublicSocialLinks>,
}

impl From<&FullProfile> for PublicProfile {
    fn from(full: &FullProfile) -> Self {
        let projects = full
            .projects
            .iter()
            .filter(|project| project.public_display)
            .map(|project| PublicProject {
                title: project.title.clone(),
                short_description: project.short_description.clone(),
                category: project.category.clone(),
                tech_stack: project.tech_stack.clone(),
                github_link: project.github_link.clone(),
                hf_link: project.hf_link.clone(),
                live_demo_link: project.live_demo_link.clone(),
                image_path: project.image_path.clone(),
                image_paths: project.image_paths.clone(),
                video_path: project.video_path.clone(),
                featured: project.featured,
            })
            .collect();

        Self {
            name: full.name.clone(),
            display_name: full.display_name.clone(),
            role: full.role.clone(),
            tagline: full.tagline.clone(),
            location: full.location.clone(),
            bio: full.bio.clone(),
            profile_image_path: full.profile_image_path.clone(),
            skills: full.skills.clone(),
            focus_areas: full.focus_areas.clone(),
            projects,
            certificates: full.certificates.clone(),
            education: full.education.clone(),
            social_links: full.social_links.as_ref().map(|links| PublicSocialLinks {
                github: links.github.clone(),
                linkedin: links.linkedin.clone(),
                email: links.email.clone(),
                phone: links.phone.clone(),
                website: links.website.clone(),
                huggingface: links.huggingface.clone(),
                kaggle: links.kaggle.clone(),
                resume: links.resume.clone(),
            }),
        }
    }
}

impl From<FullProfile> for PublicProfile {
    fn from(full: FullProfile) -> Self {
        Self::from(&full)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileValidationError {
    field: String,
    message: String,
}

impl ProfileValidationError {
    fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ProfileValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}

impl Error for ProfileValidationError {}

impl FullProfile {
    pub fn validate(&self) -> Result<(), ProfileValidationError> {
        let serialized_size = serde_json::to_vec(self)
            .map_err(|_| ProfileValidationError::new("profile", "could not be serialized"))?
            .len();
        if serialized_size > MAX_PROFILE_SERIALIZED_BYTES {
            return Err(ProfileValidationError::new(
                "profile",
                format!("must be at most {MAX_PROFILE_SERIALIZED_BYTES} serialized bytes"),
            ));
        }

        validate_optional_text("name", self.name.as_deref(), 120)?;
        validate_optional_text("display_name", self.display_name.as_deref(), 120)?;
        validate_optional_text("role", self.role.as_deref(), 160)?;
        validate_optional_text("tagline", self.tagline.as_deref(), 320)?;
        validate_optional_text("location", self.location.as_deref(), 160)?;
        validate_optional_text("bio", self.bio.as_deref(), 12_000)?;
        validate_optional_media_path("profile_image_path", self.profile_image_path.as_deref())?;
        validate_count("skills", self.skills.len(), 128)?;
        validate_count("focus_areas", self.focus_areas.len(), 64)?;
        validate_count("projects", self.projects.len(), 128)?;
        validate_count("certificates", self.certificates.len(), 256)?;
        validate_count("education", self.education.len(), 64)?;
        validate_optional_text("chatbot_rules", self.chatbot_rules.as_deref(), 16_000)?;
        validate_count("extra", self.extra.len(), 64)?;

        for key in self.extra.keys() {
            validate_text("extra key", key, 128)?;
        }

        let extra_size = serde_json::to_vec(&self.extra)
            .map_err(|_| ProfileValidationError::new("extra", "could not be serialized"))?
            .len();
        if extra_size > 128 * 1024 {
            return Err(ProfileValidationError::new(
                "extra",
                "must be at most 131072 serialized bytes",
            ));
        }

        for (index, skill) in self.skills.iter().enumerate() {
            validate_text(&format!("skills[{index}]"), skill, 160)?;
        }

        for (index, focus_area) in self.focus_areas.iter().enumerate() {
            validate_text(&format!("focus_areas[{index}]"), focus_area, 320)?;
        }

        for (index, project) in self.projects.iter().enumerate() {
            validate_project(project, index)?;
        }

        for (index, certificate) in self.certificates.iter().enumerate() {
            validate_certificate(certificate, index)?;
        }

        for (index, education) in self.education.iter().enumerate() {
            validate_education(education, index)?;
        }

        if let Some(links) = &self.social_links {
            validate_social_links(links)?;
        }

        Ok(())
    }
}

fn validate_project(project: &Project, index: usize) -> Result<(), ProfileValidationError> {
    let field = |name: &str| format!("projects[{index}].{name}");

    validate_optional_text(&field("title"), project.title.as_deref(), 240)?;
    validate_optional_text(
        &field("short_description"),
        project.short_description.as_deref(),
        6_000,
    )?;
    validate_optional_text(&field("category"), project.category.as_deref(), 160)?;
    validate_count(&field("tech_stack"), project.tech_stack.len(), 64)?;
    validate_count(&field("image_paths"), project.image_paths.len(), 32)?;

    for (tech_index, technology) in project.tech_stack.iter().enumerate() {
        validate_text(
            &format!("projects[{index}].tech_stack[{tech_index}]"),
            technology,
            120,
        )?;
    }

    validate_optional_http_url(&field("github_link"), project.github_link.as_deref())?;
    validate_optional_http_url(&field("hf_link"), project.hf_link.as_deref())?;
    validate_optional_http_url(&field("live_demo_link"), project.live_demo_link.as_deref())?;
    validate_optional_media_path(&field("image_path"), project.image_path.as_deref())?;
    validate_optional_media_path(&field("video_path"), project.video_path.as_deref())?;

    for (path_index, path) in project.image_paths.iter().enumerate() {
        validate_media_path(
            &format!("projects[{index}].image_paths[{path_index}]"),
            path,
        )?;
    }

    validate_optional_text(
        &field("internal_chatbot_notes"),
        project.internal_chatbot_notes.as_deref(),
        12_000,
    )?;
    validate_optional_text(&field("safe_notes"), project.safe_notes.as_deref(), 12_000)?;

    Ok(())
}

fn validate_certificate(
    certificate: &Certificate,
    index: usize,
) -> Result<(), ProfileValidationError> {
    let field = |name: &str| format!("certificates[{index}].{name}");

    validate_optional_text(&field("name"), certificate.name.as_deref(), 320)?;
    validate_optional_text(&field("issuer"), certificate.issuer.as_deref(), 240)?;
    validate_optional_text(&field("year"), certificate.year.as_deref(), 32)?;
    validate_optional_text(&field("date"), certificate.date.as_deref(), 64)?;
    validate_optional_http_url(&field("link"), certificate.link.as_deref())?;
    validate_optional_media_path(&field("image_path"), certificate.image_path.as_deref())?;

    Ok(())
}

fn validate_education(education: &Education, index: usize) -> Result<(), ProfileValidationError> {
    let field = |name: &str| format!("education[{index}].{name}");

    validate_optional_text(&field("institution"), education.institution.as_deref(), 320)?;
    validate_optional_text(&field("degree"), education.degree.as_deref(), 320)?;
    validate_optional_text(&field("duration"), education.duration.as_deref(), 120)?;
    validate_optional_text(&field("year"), education.year.as_deref(), 32)?;
    validate_optional_text(&field("grade"), education.grade.as_deref(), 120)?;
    validate_optional_text(&field("status"), education.status.as_deref(), 120)?;
    validate_optional_http_url(&field("link"), education.link.as_deref())?;

    Ok(())
}

fn validate_social_links(links: &SocialLinks) -> Result<(), ProfileValidationError> {
    validate_optional_http_url("social_links.github", links.github.as_deref())?;
    validate_optional_http_url("social_links.linkedin", links.linkedin.as_deref())?;
    validate_optional_email("social_links.email", links.email.as_deref())?;
    validate_optional_phone("social_links.phone", links.phone.as_deref())?;
    validate_optional_http_url("social_links.website", links.website.as_deref())?;
    validate_optional_http_url("social_links.huggingface", links.huggingface.as_deref())?;
    validate_optional_http_url("social_links.kaggle", links.kaggle.as_deref())?;
    validate_optional_http_url("social_links.resume", links.resume.as_deref())?;
    validate_optional_http_url("social_links.instagram", links.instagram.as_deref())?;

    Ok(())
}

fn validate_count(field: &str, count: usize, maximum: usize) -> Result<(), ProfileValidationError> {
    if count > maximum {
        return Err(ProfileValidationError::new(
            field,
            format!("must contain at most {maximum} items"),
        ));
    }

    Ok(())
}

fn validate_optional_text(
    field: &str,
    value: Option<&str>,
    maximum: usize,
) -> Result<(), ProfileValidationError> {
    if let Some(value) = value {
        validate_text(field, value, maximum)?;
    }

    Ok(())
}

fn validate_text(field: &str, value: &str, maximum: usize) -> Result<(), ProfileValidationError> {
    if value.chars().count() > maximum {
        return Err(ProfileValidationError::new(
            field,
            format!("must be at most {maximum} characters"),
        ));
    }

    if value.chars().any(|character| {
        character == '\0' || (character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
    }) {
        return Err(ProfileValidationError::new(
            field,
            "contains a disallowed control character",
        ));
    }

    Ok(())
}

fn validate_optional_http_url(
    field: &str,
    value: Option<&str>,
) -> Result<(), ProfileValidationError> {
    let Some(value) = non_empty(value) else {
        return Ok(());
    };

    validate_text(field, value, 2_048)?;
    if !is_http_url(value) {
        return Err(ProfileValidationError::new(
            field,
            "must be an absolute http or https URL",
        ));
    }

    Ok(())
}

fn validate_optional_media_path(
    field: &str,
    value: Option<&str>,
) -> Result<(), ProfileValidationError> {
    let Some(value) = non_empty(value) else {
        return Ok(());
    };

    validate_media_path(field, value)
}

fn validate_media_path(field: &str, value: &str) -> Result<(), ProfileValidationError> {
    let value = value.trim();
    validate_text(field, value, 2_048)?;

    let is_local_media = (value.starts_with("/media/") || value.starts_with("media/"))
        && !value.contains("..")
        && !value.contains('\\')
        && !value.contains(['?', '#'])
        && !value.chars().any(char::is_whitespace);

    if !is_local_media && !is_http_url(value) {
        return Err(ProfileValidationError::new(
            field,
            "must be a /media path or an absolute http or https URL",
        ));
    }

    Ok(())
}

fn validate_optional_email(field: &str, value: Option<&str>) -> Result<(), ProfileValidationError> {
    let Some(value) = non_empty(value) else {
        return Ok(());
    };

    validate_text(field, value, 320)?;
    let address = value.strip_prefix("mailto:").unwrap_or(value);
    let mut parts = address.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();
    let is_valid = !local.is_empty()
        && !domain.is_empty()
        && parts.next().is_none()
        && !address.chars().any(char::is_whitespace)
        && !address.contains(['/', '\\', '?', '#']);

    if !is_valid {
        return Err(ProfileValidationError::new(
            field,
            "must be an email address or mailto address",
        ));
    }

    Ok(())
}

fn validate_optional_phone(field: &str, value: Option<&str>) -> Result<(), ProfileValidationError> {
    let Some(value) = non_empty(value) else {
        return Ok(());
    };

    validate_text(field, value, 64)?;
    let number = value.strip_prefix("tel:").unwrap_or(value);
    let is_valid = number.chars().any(|character| character.is_ascii_digit())
        && number.chars().all(|character| {
            character.is_ascii_digit() || matches!(character, '+' | '-' | '(' | ')' | ' ' | '.')
        });

    if !is_valid {
        return Err(ProfileValidationError::new(
            field,
            "must be a phone number or tel address",
        ));
    }

    Ok(())
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn is_http_url(value: &str) -> bool {
    let value = value.trim();
    if value.chars().any(char::is_whitespace) {
        return false;
    }

    let lower = value.to_ascii_lowercase();
    let remainder = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"));

    let Some(remainder) = remainder else {
        return false;
    };

    let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default();

    !authority.is_empty() && authority != "." && authority != ".."
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_profile() -> FullProfile {
        FullProfile {
            name: Some("Chamira Hashan".to_string()),
            profile_image_path: Some("/media/projects/images/profile.webp".to_string()),
            skills: vec!["Rust".to_string()],
            projects: vec![Project {
                title: Some("Secure portfolio".to_string()),
                github_link: Some("https://github.com/example/portfolio".to_string()),
                image_paths: vec!["/media/projects/images/project.webp".to_string()],
                public_display: true,
                chatbot_visible: true,
                ..Project::default()
            }],
            social_links: Some(SocialLinks {
                email: Some("hello@example.com".to_string()),
                phone: Some("tel:+94 77 123 4567".to_string()),
                github: Some("https://github.com/example".to_string()),
                instagram: Some("https://instagram.com/private".to_string()),
                ..SocialLinks::default()
            }),
            ..FullProfile::default()
        }
    }

    #[test]
    fn missing_chatbot_visibility_defaults_to_private() {
        let project: Project = serde_json::from_str(r#"{"public_display":false}"#).unwrap();

        assert!(!project.chatbot_visible);
    }

    #[test]
    fn public_profile_omits_instagram_and_private_projects() {
        let mut profile = valid_profile();
        profile.projects.push(Project {
            title: Some("Private project".to_string()),
            public_display: false,
            chatbot_visible: true,
            ..Project::default()
        });

        let public = PublicProfile::from(profile.clone());
        let public_json = serde_json::to_value(public).unwrap();
        let admin_json = serde_json::to_value(profile).unwrap();

        assert_eq!(public_json["projects"].as_array().unwrap().len(), 1);
        assert!(public_json["social_links"].get("instagram").is_none());
        assert_eq!(
            admin_json["social_links"]["instagram"],
            "https://instagram.com/private"
        );
    }

    #[test]
    fn validates_supported_urls_contacts_and_media_paths() {
        assert!(valid_profile().validate().is_ok());
    }

    #[test]
    fn rejects_unsafe_url_schemes() {
        let mut profile = valid_profile();
        profile.projects[0].live_demo_link = Some("javascript:alert(1)".to_string());

        let error = profile.validate().unwrap_err();
        assert_eq!(error.field(), "projects[0].live_demo_link");
    }

    #[test]
    fn rejects_media_path_traversal() {
        let mut profile = valid_profile();
        profile.profile_image_path = Some("/media/../profile.json".to_string());

        let error = profile.validate().unwrap_err();
        assert_eq!(error.field(), "profile_image_path");
    }

    #[test]
    fn rejects_control_characters_and_oversized_collections() {
        let mut profile = valid_profile();
        profile.name = Some("bad\u{0}name".to_string());
        assert_eq!(profile.validate().unwrap_err().field(), "name");

        profile.name = None;
        profile.skills = vec!["Rust".to_string(); 129];
        assert_eq!(profile.validate().unwrap_err().field(), "skills");
    }

    #[test]
    fn rejects_oversized_serialized_profiles() {
        let mut profile = valid_profile();
        profile.extra.insert(
            "oversized".to_string(),
            serde_json::Value::String("x".repeat(MAX_PROFILE_SERIALIZED_BYTES)),
        );

        let error = profile.validate().unwrap_err();
        assert_eq!(error.field(), "profile");
    }
}
