use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    #[serde(default = "default_true")]
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
    pub social_links: Option<SocialLinks>,
}

impl From<FullProfile> for PublicProfile {
    fn from(full: FullProfile) -> Self {
        let projects = full
            .projects
            .into_iter()
            .filter(|project| project.public_display)
            .map(|project| PublicProject {
                title: project.title,
                short_description: project.short_description,
                category: project.category,
                tech_stack: project.tech_stack,
                github_link: project.github_link,
                hf_link: project.hf_link,
                live_demo_link: project.live_demo_link,
                image_path: project.image_path,
                image_paths: project.image_paths,
                video_path: project.video_path,
                featured: project.featured,
            })
            .collect();

        Self {
            name: full.name,
            display_name: full.display_name,
            role: full.role,
            tagline: full.tagline,
            location: full.location,
            bio: full.bio,
            profile_image_path: full.profile_image_path,
            skills: full.skills,
            focus_areas: full.focus_areas,
            projects,
            certificates: full.certificates,
            education: full.education,
            social_links: full.social_links,
        }
    }
}
