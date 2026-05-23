use axum::{Json, extract::Multipart, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    auth::{AuthError, AuthResponse, generate_token},
    profile::FullProfile,
    storage::{load_profile, project_images_path, project_videos_path, save_profile},
};

const MAX_UPLOAD_SIZE_BYTES: usize = 25 * 1024 * 1024;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AdminStatusResponse {
    pub authenticated: bool,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthError>)> {
    let admin_email = env::var("ADMIN_EMAIL").unwrap_or_default();
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_default();

    if admin_email.is_empty() || admin_password.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError {
                error: "Admin credentials are not configured.".to_string(),
            }),
        ));
    }

    if payload.email.trim() != admin_email || payload.password != admin_password {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "Invalid email or password.".to_string(),
            }),
        ));
    }

    generate_token(&admin_email)
        .map(|token| Json(AuthResponse { token }))
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
}

pub async fn verify_admin_handler() -> Json<AdminStatusResponse> {
    Json(AdminStatusResponse {
        authenticated: true,
    })
}

pub async fn get_admin_profile_handler() -> Result<Json<FullProfile>, (StatusCode, String)> {
    load_profile()
        .map(Json)
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
}

pub async fn update_admin_profile_handler(
    Json(payload): Json<FullProfile>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    save_profile(&payload)
        .map(|_| Json(serde_json::json!({ "message": "Profile updated successfully." })))
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
}

pub async fn upload_media_handler(
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut saved_files = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
    {
        let file_name = match field.file_name() {
            Some(file_name) => sanitize_file_name(file_name),
            None => continue,
        };

        let content_type = field.content_type().unwrap_or("").to_string();
        let upload_kind = detect_upload_kind(&content_type, &file_name).ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Only PNG, JPG, JPEG, WEBP, GIF, MP4, WEBM, and MOV files are allowed.".to_string(),
            )
        })?;

        let data = field
            .bytes()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?;

        if data.len() > MAX_UPLOAD_SIZE_BYTES {
            return Err((
                StatusCode::PAYLOAD_TOO_LARGE,
                "File is too large. Maximum allowed size is 25MB.".to_string(),
            ));
        }

        let upload_dir = match upload_kind {
            UploadKind::Image => project_images_path(),
            UploadKind::Video => project_videos_path(),
        };

        fs::create_dir_all(&upload_dir)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

        let final_name = unique_file_name(&upload_dir, &file_name).await;
        let file_path = upload_dir.join(&final_name);

        let mut file = fs::File::create(&file_path)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

        file.write_all(&data)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

        let public_url = match upload_kind {
            UploadKind::Image => format!("/media/projects/images/{}", final_name),
            UploadKind::Video => format!("/media/projects/videos/{}", final_name),
        };

        saved_files.push(public_url);
    }

    Ok(Json(serde_json::json!({
        "message": "Upload successful.",
        "files": saved_files
    })))
}

#[derive(Clone, Copy)]
enum UploadKind {
    Image,
    Video,
}

fn detect_upload_kind(content_type: &str, file_name: &str) -> Option<UploadKind> {
    let lower_name = file_name.to_lowercase();

    let is_image = content_type.starts_with("image/")
        || lower_name.ends_with(".png")
        || lower_name.ends_with(".jpg")
        || lower_name.ends_with(".jpeg")
        || lower_name.ends_with(".webp")
        || lower_name.ends_with(".gif");

    let is_video = content_type.starts_with("video/")
        || lower_name.ends_with(".mp4")
        || lower_name.ends_with(".webm")
        || lower_name.ends_with(".mov");

    if is_image {
        Some(UploadKind::Image)
    } else if is_video {
        Some(UploadKind::Video)
    } else {
        None
    }
}

fn sanitize_file_name(file_name: &str) -> String {
    let cleaned = file_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string();

    if cleaned.is_empty() {
        "upload.bin".to_string()
    } else {
        cleaned
    }
}

async fn unique_file_name(directory: &PathBuf, file_name: &str) -> String {
    let mut candidate = file_name.to_string();
    let mut counter = 1;

    while directory.join(&candidate).exists() {
        let path = std::path::Path::new(file_name);
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("file");
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");

        candidate = if extension.is_empty() {
            format!("{}_{}", stem, counter)
        } else {
            format!("{}_{}.{}", stem, counter, extension)
        };

        counter += 1;
    }

    candidate
}
