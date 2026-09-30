use axum::{
    Json,
    body::Bytes,
    extract::{
        Multipart, State,
        multipart::{Field, MultipartRejection},
        rejection::JsonRejection,
    },
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{
    auth::{self, AuthResponse},
    error::ApiError,
    profile::FullProfile,
    state::AppState,
    storage::{
        SaveProfileError, assets_path, project_images_path, project_videos_path,
        upload_staging_path,
    },
};

pub const MAX_UPLOAD_SIZE_BYTES: usize = 25 * 1024 * 1024;
pub const MAX_UPLOAD_REQUEST_BYTES: usize = MAX_UPLOAD_SIZE_BYTES + 128 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AdminStatusResponse {
    pub authenticated: bool,
}

pub async fn login_handler(
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Json<AuthResponse>, ApiError> {
    let Json(payload) = payload.map_err(json_rejection)?;
    let authentication = tokio::task::spawn_blocking(move || {
        auth::authenticate_admin(&payload.email, &payload.password)
    })
    .await
    .map_err(|error| ApiError::internal("admin_login_task", error))?;

    match authentication {
        Ok(response) => Ok(Json(response)),
        Err(error) if error.is_invalid() => Err(ApiError::invalid_credentials()),
        Err(error) => Err(ApiError::internal(
            "admin_token_generation",
            format_args!("{error:?}"),
        )),
    }
}

pub async fn verify_admin_handler() -> Json<AdminStatusResponse> {
    Json(AdminStatusResponse {
        authenticated: true,
    })
}

pub async fn get_admin_profile_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Response, ApiError> {
    let snapshot = state.profiles.snapshot().await;
    let mut response = Json((*snapshot.profile).clone()).into_response();
    insert_etag(response.headers_mut(), &snapshot.etag)?;
    Ok(response)
}

pub async fn update_admin_profile_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    payload: Result<Json<FullProfile>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(payload) = payload.map_err(json_rejection)?;
    payload.validate().map_err(|error| {
        ApiError::bad_request(format!("Invalid {}: {}", error.field(), error.message()))
    })?;

    let expected_etag = headers
        .get(header::IF_MATCH)
        .map(|value| {
            value
                .to_str()
                .map_err(|_| ApiError::bad_request("If-Match must be a valid ETag."))
        })
        .transpose()?;

    let snapshot =
        state
            .profiles
            .save(payload, expected_etag)
            .await
            .map_err(|error| match error {
                SaveProfileError::Conflict => ApiError::conflict(
                    "The profile changed after it was loaded. Reload it before saving again.",
                ),
                SaveProfileError::Persistence(error) => {
                    ApiError::internal("profile_persistence", error)
                }
            })?;

    let mut response = Json(serde_json::json!({
        "message": "Profile updated successfully."
    }))
    .into_response();
    insert_etag(response.headers_mut(), &snapshot.etag)?;
    Ok(response)
}

pub async fn upload_media_handler(
    State(state): State<Arc<AppState>>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut multipart = multipart.map_err(multipart_rejection)?;
    let _upload_guard = state.upload_lock.lock().await;
    let staging_directory = upload_staging_path();
    fs::create_dir_all(&staging_directory)
        .await
        .map_err(|error| ApiError::internal("upload_staging_directory", error))?;

    let mut staged_upload: Option<StagedUpload> = None;

    loop {
        let next_field = multipart.next_field().await;
        let field = match next_field {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(error) => {
                cleanup_staged(staged_upload.take()).await;
                tracing::warn!(error = %error, "Rejected malformed multipart upload");
                return Err(ApiError::bad_request("The multipart upload is malformed."));
            }
        };

        if field.name() != Some("file") || field.file_name().is_none() {
            cleanup_staged(staged_upload.take()).await;
            return Err(ApiError::bad_request(
                "Exactly one multipart file field named 'file' is required.",
            ));
        }

        if staged_upload.is_some() {
            cleanup_staged(staged_upload.take()).await;
            return Err(ApiError::bad_request(
                "Only one file may be uploaded at a time.",
            ));
        }

        staged_upload = Some(stage_upload(field, &staging_directory).await?);
    }

    let staged = staged_upload.ok_or_else(|| {
        ApiError::bad_request("Exactly one multipart file field named 'file' is required.")
    })?;

    let format = match MediaFormat::detect(&staged.header) {
        Some(format)
            if format.matches_declared_type(&staged.content_type)
                && format.matches_extension(&staged.extension) =>
        {
            format
        }
        _ => {
            cleanup_path(&staged.path).await;
            return Err(ApiError::bad_request(
                "The file bytes, MIME type, and extension must match an allowed PNG, JPEG, WEBP, GIF, MP4, WEBM, or MOV format.",
            ));
        }
    };

    let assets_directory = assets_path();
    let usage_task = tokio::task::spawn_blocking(move || directory_size(&assets_directory)).await;
    let current_usage = match usage_task {
        Ok(Ok(usage)) => usage,
        Ok(Err(error)) => {
            cleanup_path(&staged.path).await;
            return Err(ApiError::internal("media_quota_scan", error));
        }
        Err(error) => {
            cleanup_path(&staged.path).await;
            return Err(ApiError::internal("media_quota_task", error));
        }
    };
    let projected_usage = match current_usage.checked_add(staged.size) {
        Some(usage) => usage,
        None => {
            cleanup_path(&staged.path).await;
            return Err(ApiError::payload_too_large(
                "The media storage quota would be exceeded.",
            ));
        }
    };

    if projected_usage > state.max_media_storage_bytes {
        cleanup_path(&staged.path).await;
        return Err(ApiError::payload_too_large(
            "The media storage quota would be exceeded.",
        ));
    }

    let upload_directory = match format.kind {
        UploadKind::Image => project_images_path(),
        UploadKind::Video => project_videos_path(),
    };
    if let Err(error) = fs::create_dir_all(&upload_directory).await {
        cleanup_path(&staged.path).await;
        return Err(ApiError::internal("media_directory", error));
    }

    let final_name = format!("{}.{}", staged.id.simple(), format.extension);
    let final_path = upload_directory.join(&final_name);

    match fs::try_exists(&final_path).await {
        Ok(true) => {
            cleanup_path(&staged.path).await;
            return Err(ApiError::internal(
                "media_identifier_collision",
                "generated media identifier already exists",
            ));
        }
        Ok(false) => {}
        Err(error) => {
            cleanup_path(&staged.path).await;
            return Err(ApiError::internal("media_collision_check", error));
        }
    }

    if let Err(error) = fs::rename(&staged.path, &final_path).await {
        cleanup_path(&staged.path).await;
        return Err(ApiError::internal("media_publish", error));
    }

    let public_url = match format.kind {
        UploadKind::Image => format!("/media/projects/images/{final_name}"),
        UploadKind::Video => format!("/media/projects/videos/{final_name}"),
    };

    Ok(Json(serde_json::json!({
        "message": "Upload successful.",
        "files": [public_url]
    })))
}

fn json_rejection(rejection: JsonRejection) -> ApiError {
    if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
        ApiError::payload_too_large("The JSON request body is too large.")
    } else {
        ApiError::bad_request("The JSON request body is invalid.")
    }
}

fn multipart_rejection(rejection: MultipartRejection) -> ApiError {
    if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
        ApiError::payload_too_large("The multipart request body is too large.")
    } else {
        ApiError::bad_request("The multipart request body is invalid.")
    }
}

fn insert_etag(headers: &mut HeaderMap, etag: &str) -> Result<(), ApiError> {
    let value =
        HeaderValue::from_str(etag).map_err(|error| ApiError::internal("profile_etag", error))?;
    headers.insert(header::ETAG, value);
    Ok(())
}

struct StagedUpload {
    id: Uuid,
    path: PathBuf,
    content_type: String,
    extension: String,
    header: Vec<u8>,
    size: u64,
}

impl Drop for StagedUpload {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

struct StagingFileGuard {
    path: PathBuf,
    armed: bool,
}

impl StagingFileGuard {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for StagingFileGuard {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

async fn stage_upload(
    mut field: Field<'_>,
    staging_directory: &Path,
) -> Result<StagedUpload, ApiError> {
    let content_type = field
        .content_type()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let extension = field
        .file_name()
        .and_then(|name| std::path::Path::new(name).extension())
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let id = Uuid::new_v4();
    let path = staging_directory.join(format!("{}.upload", id.simple()));
    let mut cleanup_guard = StagingFileGuard::new(path.clone());
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
        .map_err(|error| ApiError::internal("upload_staging_file", error))?;
    let mut header_bytes = Vec::with_capacity(64);
    let mut size = 0_u64;

    while let Some(chunk) = match field.chunk().await {
        Ok(chunk) => chunk,
        Err(error) => {
            drop(output);
            cleanup_path(&path).await;
            tracing::warn!(error = %error, "Upload stream could not be read");
            return Err(ApiError::bad_request(
                "The uploaded file could not be read.",
            ));
        }
    } {
        size = match size.checked_add(chunk.len() as u64) {
            Some(size) => size,
            None => {
                drop(output);
                cleanup_path(&path).await;
                return Err(ApiError::payload_too_large(
                    "The uploaded file is too large.",
                ));
            }
        };

        if size > MAX_UPLOAD_SIZE_BYTES as u64 {
            drop(output);
            cleanup_path(&path).await;
            return Err(ApiError::payload_too_large(
                "The file is too large. Maximum allowed size is 25MB.",
            ));
        }

        capture_header(&mut header_bytes, &chunk);
        if let Err(error) = output.write_all(&chunk).await {
            drop(output);
            cleanup_path(&path).await;
            return Err(ApiError::internal("upload_write", error));
        }
    }

    if size == 0 {
        drop(output);
        cleanup_path(&path).await;
        return Err(ApiError::bad_request("The uploaded file is empty."));
    }

    if let Err(error) = output.flush().await {
        drop(output);
        cleanup_path(&path).await;
        return Err(ApiError::internal("upload_flush", error));
    }
    if let Err(error) = output.sync_all().await {
        drop(output);
        cleanup_path(&path).await;
        return Err(ApiError::internal("upload_sync", error));
    }
    drop(output);
    cleanup_guard.disarm();

    Ok(StagedUpload {
        id,
        path,
        content_type,
        extension,
        header: header_bytes,
        size,
    })
}

fn capture_header(header: &mut Vec<u8>, chunk: &Bytes) {
    const MAX_HEADER_BYTES: usize = 64;
    let remaining = MAX_HEADER_BYTES.saturating_sub(header.len());
    header.extend_from_slice(&chunk[..chunk.len().min(remaining)]);
}

async fn cleanup_staged(staged: Option<StagedUpload>) {
    if let Some(staged) = staged {
        cleanup_path(&staged.path).await;
    }
}

async fn cleanup_path(path: &Path) {
    if let Err(error) = fs::remove_file(path).await
        && error.kind() != std::io::ErrorKind::NotFound
    {
        tracing::warn!(error = %error, "Failed to remove a staged upload");
    }
}

fn directory_size(path: &Path) -> std::io::Result<u64> {
    if !path.exists() {
        return Ok(0);
    }

    let mut total = 0_u64;
    let mut pending = vec![path.to_path_buf()];

    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                pending.push(entry.path());
            } else if metadata.is_file() {
                total = total
                    .checked_add(metadata.len())
                    .ok_or_else(|| std::io::Error::other("media storage size overflowed u64"))?;
            }
        }
    }

    Ok(total)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UploadKind {
    Image,
    Video,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MediaFormat {
    kind: UploadKind,
    extension: &'static str,
    mime: &'static str,
}

impl MediaFormat {
    fn detect(bytes: &[u8]) -> Option<Self> {
        if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            return Some(Self::image("png", "image/png"));
        }
        if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
            return Some(Self::image("jpg", "image/jpeg"));
        }
        if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            return Some(Self::image("gif", "image/gif"));
        }
        if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
            return Some(Self::image("webp", "image/webp"));
        }
        if bytes.starts_with(&[0x1a, 0x45, 0xdf, 0xa3]) {
            return Some(Self::video("webm", "video/webm"));
        }
        if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" {
            if &bytes[8..12] == b"qt  " {
                return Some(Self::video("mov", "video/quicktime"));
            }
            return Some(Self::video("mp4", "video/mp4"));
        }

        None
    }

    const fn image(extension: &'static str, mime: &'static str) -> Self {
        Self {
            kind: UploadKind::Image,
            extension,
            mime,
        }
    }

    const fn video(extension: &'static str, mime: &'static str) -> Self {
        Self {
            kind: UploadKind::Video,
            extension,
            mime,
        }
    }

    fn matches_declared_type(self, declared: &str) -> bool {
        declared.eq_ignore_ascii_case(self.mime)
    }

    fn matches_extension(self, extension: &str) -> bool {
        extension.eq_ignore_ascii_case(self.extension)
            || (self.extension == "jpg" && extension.eq_ignore_ascii_case("jpeg"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_supported_magic_bytes_and_rejects_active_content() {
        let png = MediaFormat::detect(b"\x89PNG\r\n\x1a\nrest").unwrap();
        assert_eq!(png.extension, "png");
        assert!(png.matches_declared_type("image/png"));
        assert!(MediaFormat::detect(b"<svg><script>alert(1)</script></svg>").is_none());
        assert!(MediaFormat::detect(b"<!doctype html><script></script>").is_none());
    }

    #[test]
    fn requires_mime_and_extension_to_match_detected_bytes() {
        let jpeg = MediaFormat::detect(&[0xff, 0xd8, 0xff, 0xe0]).unwrap();
        assert!(jpeg.matches_declared_type("image/jpeg"));
        assert!(jpeg.matches_extension("jpeg"));
        assert!(!jpeg.matches_declared_type("image/png"));
        assert!(!jpeg.matches_extension("html"));
    }

    #[test]
    fn differentiates_quicktime_and_mp4_brands() {
        let mov = MediaFormat::detect(b"\x00\x00\x00\x14ftypqt  rest").unwrap();
        let mp4 = MediaFormat::detect(b"\x00\x00\x00\x14ftypisomrest").unwrap();
        assert_eq!(mov.extension, "mov");
        assert_eq!(mp4.extension, "mp4");
    }
}
