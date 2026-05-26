use crate::profile::FullProfile;
use anyhow::{Context, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn storage_root() -> PathBuf {
    if Path::new("/data").exists() {
        PathBuf::from("/data")
    } else {
        PathBuf::from("./data")
    }
}

pub fn profile_path() -> PathBuf {
    storage_root()
        .join("profile")
        .join("portfolio_profile.json")
}

pub fn assets_path() -> PathBuf {
    storage_root().join("assets")
}

pub fn project_images_path() -> PathBuf {
    assets_path().join("projects").join("images")
}

pub fn project_videos_path() -> PathBuf {
    assets_path().join("projects").join("videos")
}

pub fn load_profile() -> Result<FullProfile> {
    let path = profile_path();

    if path.exists() {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read profile from {}", path.display()))?;

        return serde_json::from_str::<FullProfile>(&content)
            .with_context(|| format!("Failed to parse profile JSON from {}", path.display()));
    }

    if let Ok(env_json) = env::var("PORTFOLIO_PROFILE_JSON") {
        if !env_json.trim().is_empty() {
            let profile = serde_json::from_str::<FullProfile>(&env_json)
                .with_context(|| "Failed to parse PORTFOLIO_PROFILE_JSON")?;

            let _ = save_profile(&profile);

            return Ok(profile);
        }
    }

    anyhow::bail!("Profile data was not found in bucket storage or environment configuration")
}

pub fn save_profile(profile: &FullProfile) -> Result<()> {
    let path = profile_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create profile directory {}", parent.display()))?;
    }

    let content =
        serde_json::to_string_pretty(profile).with_context(|| "Failed to serialize profile")?;

    let temp_path = path.with_extension("json.tmp");

    fs::write(&temp_path, content)
        .with_context(|| format!("Failed to write temp profile {}", temp_path.display()))?;

    fs::rename(&temp_path, &path)
        .with_context(|| format!("Failed to replace profile {}", path.display()))?;

    Ok(())
}
