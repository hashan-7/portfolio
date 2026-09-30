use crate::profile::FullProfile;
use anyhow::{Context, Result};
use std::{
    collections::hash_map::DefaultHasher,
    env, fs,
    hash::{Hash, Hasher},
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

const PROFILE_FILE_NAME: &str = "portfolio_profile.json";
const PROFILE_BACKUP_FILE_NAME: &str = "portfolio_profile.backup.json";

#[derive(Clone)]
pub struct ProfileStore {
    inner: Arc<ProfileStoreInner>,
}

struct ProfileStoreInner {
    snapshot: RwLock<ProfileSnapshot>,
    write_lock: Mutex<()>,
    profile_path: PathBuf,
}

#[derive(Clone)]
pub struct ProfileSnapshot {
    pub profile: Arc<FullProfile>,
    pub etag: String,
}

#[derive(Debug)]
pub enum SaveProfileError {
    Conflict,
    Persistence(anyhow::Error),
}

impl std::fmt::Display for SaveProfileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict => formatter.write_str("the profile has changed since it was loaded"),
            Self::Persistence(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for SaveProfileError {}

impl ProfileStore {
    pub async fn initialize() -> Result<Self> {
        Self::initialize_at(profile_path()).await
    }

    async fn initialize_at(path: PathBuf) -> Result<Self> {
        let load_path = path.clone();
        let profile = tokio::task::spawn_blocking(move || load_or_seed_profile(&load_path))
            .await
            .context("Profile initialization task failed")??;
        profile
            .validate()
            .context("Stored profile failed validation")?;
        let snapshot = snapshot_from(profile)?;

        Ok(Self {
            inner: Arc::new(ProfileStoreInner {
                snapshot: RwLock::new(snapshot),
                write_lock: Mutex::new(()),
                profile_path: path,
            }),
        })
    }

    pub async fn snapshot(&self) -> ProfileSnapshot {
        self.inner.snapshot.read().await.clone()
    }

    #[cfg(test)]
    pub fn for_test(profile: FullProfile) -> Self {
        Self {
            inner: Arc::new(ProfileStoreInner {
                snapshot: RwLock::new(
                    snapshot_from(profile).expect("test profile must serialize successfully"),
                ),
                write_lock: Mutex::new(()),
                profile_path: env::temp_dir().join(format!(
                    "portfolio-profile-test-{}.json",
                    Uuid::new_v4().simple()
                )),
            }),
        }
    }

    pub async fn save(
        &self,
        profile: FullProfile,
        expected_etag: Option<&str>,
    ) -> std::result::Result<ProfileSnapshot, SaveProfileError> {
        let store = self.clone();
        let expected_etag = expected_etag.map(str::to_owned);

        tokio::spawn(async move { store.commit(profile, expected_etag.as_deref()).await })
            .await
            .map_err(|error| {
                SaveProfileError::Persistence(
                    anyhow::Error::new(error).context("Profile commit task failed"),
                )
            })?
    }

    async fn commit(
        &self,
        profile: FullProfile,
        expected_etag: Option<&str>,
    ) -> std::result::Result<ProfileSnapshot, SaveProfileError> {
        let _write_guard = self.inner.write_lock.lock().await;

        if let Some(expected_etag) = expected_etag {
            let current = self.inner.snapshot.read().await;
            if !constant_time_eq(expected_etag.as_bytes(), current.etag.as_bytes()) {
                return Err(SaveProfileError::Conflict);
            }
        }

        let next_snapshot = snapshot_from(profile).map_err(SaveProfileError::Persistence)?;
        let write_profile = Arc::clone(&next_snapshot.profile);
        let write_path = self.inner.profile_path.clone();

        tokio::task::spawn_blocking(move || persist_profile(&write_path, &write_profile))
            .await
            .map_err(|error| {
                SaveProfileError::Persistence(
                    anyhow::Error::new(error).context("Profile write task failed"),
                )
            })?
            .map_err(SaveProfileError::Persistence)?;

        *self.inner.snapshot.write().await = next_snapshot.clone();
        Ok(next_snapshot)
    }
}

fn storage_root() -> PathBuf {
    if let Ok(configured) = env::var("PORTFOLIO_DATA_DIR") {
        let configured = configured.trim();
        if !configured.is_empty() {
            return PathBuf::from(configured);
        }
    }

    if Path::new("/data").exists() {
        PathBuf::from("/data")
    } else {
        PathBuf::from("./data")
    }
}

pub fn profile_path() -> PathBuf {
    storage_root().join("profile").join(PROFILE_FILE_NAME)
}

fn profile_backup_path(path: &Path) -> PathBuf {
    path.with_file_name(PROFILE_BACKUP_FILE_NAME)
}

pub fn assets_path() -> PathBuf {
    storage_root().join("assets")
}

pub fn upload_staging_path() -> PathBuf {
    storage_root().join(".upload-staging")
}

pub async fn cleanup_upload_staging() -> Result<()> {
    let directory = upload_staging_path();
    let mut entries = match tokio::fs::read_dir(&directory).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("Failed to inspect {}", directory.display()));
        }
    };

    while let Some(entry) = entries
        .next_entry()
        .await
        .with_context(|| format!("Failed to inspect {}", directory.display()))?
    {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if !is_staging_file_name(name) {
            continue;
        }

        let file_type = entry.file_type().await.with_context(|| {
            format!("Failed to inspect stale upload {}", entry.path().display())
        })?;
        if file_type.is_file() || file_type.is_symlink() {
            tokio::fs::remove_file(entry.path())
                .await
                .with_context(|| {
                    format!("Failed to remove stale upload {}", entry.path().display())
                })?;
        }
    }

    Ok(())
}

fn is_staging_file_name(name: &str) -> bool {
    name.strip_suffix(".upload")
        .is_some_and(|stem| stem.len() == 32 && stem.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

pub fn project_images_path() -> PathBuf {
    assets_path().join("projects").join("images")
}

pub fn project_videos_path() -> PathBuf {
    assets_path().join("projects").join("videos")
}

fn load_or_seed_profile(path: &Path) -> Result<FullProfile> {
    if path.exists() {
        match load_validated_profile_file(path) {
            Ok(profile) => return Ok(profile),
            Err(primary_error) => {
                let backup_path = profile_backup_path(path);
                if !backup_path.exists() {
                    return Err(primary_error);
                }

                tracing::warn!(
                    error = %primary_error,
                    "Primary profile is invalid; restoring the last known-good backup"
                );
                let backup = load_validated_profile_file(&backup_path)
                    .context("Primary profile was invalid and the backup could not be loaded")?;
                let backup_bytes = fs::read(&backup_path).with_context(|| {
                    format!("Failed to read backup profile {}", backup_path.display())
                })?;
                atomic_write(path, &backup_bytes)
                    .context("Failed to restore the last known-good profile backup")?;
                return Ok(backup);
            }
        }
    }

    let backup_path = profile_backup_path(path);
    if backup_path.exists() {
        tracing::warn!(
            backup = %backup_path.display(),
            "Primary profile is missing; restoring the last known-good backup"
        );
        let backup = load_validated_profile_file(&backup_path)
            .context("Primary profile was missing and the backup could not be loaded")?;
        let backup_bytes = fs::read(&backup_path)
            .with_context(|| format!("Failed to read backup profile {}", backup_path.display()))?;
        atomic_write(path, &backup_bytes)
            .context("Failed to restore the missing profile from backup")?;
        return Ok(backup);
    }

    if let Ok(env_json) = env::var("PORTFOLIO_PROFILE_JSON")
        && !env_json.trim().is_empty()
    {
        let profile = serde_json::from_str::<FullProfile>(&env_json)
            .context("Failed to parse PORTFOLIO_PROFILE_JSON")?;
        profile
            .validate()
            .context("PORTFOLIO_PROFILE_JSON failed validation")?;
        persist_profile(path, &profile)
            .context("Profile seed was valid but could not be persisted")?;
        return Ok(profile);
    }

    anyhow::bail!("Profile data was not found in persistent storage or PORTFOLIO_PROFILE_JSON")
}

fn load_profile_file(path: &Path) -> Result<FullProfile> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read profile from {}", path.display()))?;

    serde_json::from_str::<FullProfile>(&content)
        .with_context(|| format!("Failed to parse profile JSON from {}", path.display()))
}

fn load_validated_profile_file(path: &Path) -> Result<FullProfile> {
    let profile = load_profile_file(path)?;
    profile
        .validate()
        .with_context(|| format!("Profile validation failed for {}", path.display()))?;
    Ok(profile)
}

fn persist_profile(path: &Path, profile: &FullProfile) -> Result<()> {
    let parent = path
        .parent()
        .context("Profile path does not have a parent directory")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create profile directory {}", parent.display()))?;

    let content = serde_json::to_vec_pretty(profile).context("Failed to serialize profile")?;

    if path.exists() {
        let previous = fs::read(path)
            .with_context(|| format!("Failed to read current profile {}", path.display()))?;
        let previous_profile = serde_json::from_slice::<FullProfile>(&previous)
            .context("Refusing to replace an invalid current profile")?;
        previous_profile
            .validate()
            .context("Refusing to replace a current profile that fails validation")?;
        atomic_write(&profile_backup_path(path), &previous)
            .context("Failed to create the last known-good profile backup")?;
    }

    atomic_write(path, &content)
        .with_context(|| format!("Failed to atomically replace profile {}", path.display()))
}

fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .context("Atomic write path does not have a parent directory")?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("profile");
    let temporary_path = parent.join(format!(".{file_name}.{}.tmp", Uuid::new_v4().simple()));
    let result = (|| -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
            .with_context(|| {
                format!(
                    "Failed to create temporary profile file {}",
                    temporary_path.display()
                )
            })?;
        file.write_all(content)
            .with_context(|| format!("Failed to write {}", temporary_path.display()))?;
        file.sync_all()
            .with_context(|| format!("Failed to sync {}", temporary_path.display()))?;
        drop(file);
        replace_file(&temporary_path, path)?;
        if let Err(error) = sync_directory(parent) {
            tracing::warn!(
                error = %error,
                directory = %parent.display(),
                "Profile was committed, but the storage backend could not sync its directory entry"
            );
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    result.with_context(|| format!("Failed to commit {}", path.display()))
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> std::io::Result<()> {
    fs::File::open(path)?.sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

#[cfg(not(windows))]
fn replace_file(temporary_path: &Path, destination: &Path) -> std::io::Result<()> {
    fs::rename(temporary_path, destination)
}

#[cfg(windows)]
fn replace_file(temporary_path: &Path, destination: &Path) -> std::io::Result<()> {
    match fs::rename(temporary_path, destination) {
        Ok(()) => return Ok(()),
        Err(_) if destination.exists() => {}
        Err(error) => return Err(error),
    }

    let parent = destination.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "destination has no parent directory",
        )
    })?;
    let displaced_path = parent.join(format!(".profile.{}.old", Uuid::new_v4().simple()));
    fs::rename(destination, &displaced_path)?;

    match fs::rename(temporary_path, destination) {
        Ok(()) => {
            let _ = fs::remove_file(displaced_path);
            Ok(())
        }
        Err(error) => {
            let _ = fs::rename(displaced_path, destination);
            Err(error)
        }
    }
}

fn snapshot_from(profile: FullProfile) -> Result<ProfileSnapshot> {
    let serialized =
        serde_json::to_vec(&profile).context("Failed to serialize profile snapshot")?;
    let mut hasher = DefaultHasher::new();
    serialized.hash(&mut hasher);
    let etag = format!("\"{:016x}\"", hasher.finish());

    Ok(ProfileSnapshot {
        profile: Arc::new(profile),
        etag,
    })
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let max_len = left.len().max(right.len());
    let mut difference = left.len() ^ right.len();

    for index in 0..max_len {
        let left_byte = left.get(index).copied().unwrap_or_default();
        let right_byte = right.get(index).copied().unwrap_or_default();
        difference |= usize::from(left_byte ^ right_byte);
    }

    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_profile_path() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        env::temp_dir()
            .join(format!("portfolio-storage-test-{unique}"))
            .join("profile")
            .join(PROFILE_FILE_NAME)
    }

    #[tokio::test]
    async fn serializes_updates_and_rejects_stale_etags() {
        let path = temporary_profile_path();
        let initial = FullProfile {
            name: Some("Initial".to_string()),
            ..FullProfile::default()
        };
        persist_profile(&path, &initial).expect("initial profile should persist");
        let store = ProfileStore::initialize_at(path.clone())
            .await
            .expect("store should initialize");
        let first = store.snapshot().await;

        let mut updated = (*first.profile).clone();
        updated.name = Some("Updated".to_string());
        let second = store
            .save(updated, Some(&first.etag))
            .await
            .expect("matching etag should save");

        let stale_result = store
            .save((*first.profile).clone(), Some(&first.etag))
            .await;
        assert!(matches!(stale_result, Err(SaveProfileError::Conflict)));
        assert_ne!(first.etag, second.etag);

        let reloaded = load_profile_file(&path).expect("saved profile should be valid");
        assert_eq!(reloaded.name.as_deref(), Some("Updated"));
        let _ = fs::remove_dir_all(path.parent().and_then(Path::parent).unwrap());
    }

    #[tokio::test]
    async fn restores_last_known_good_backup_when_primary_is_corrupt() {
        let path = temporary_profile_path();
        let first = FullProfile {
            name: Some("Last known good".to_string()),
            ..FullProfile::default()
        };
        persist_profile(&path, &first).expect("first profile should persist");
        let second = FullProfile {
            name: Some("Current".to_string()),
            ..FullProfile::default()
        };
        persist_profile(&path, &second).expect("second profile should persist");
        fs::write(&path, b"{broken json").expect("test should corrupt the primary");

        let store = ProfileStore::initialize_at(path.clone())
            .await
            .expect("valid backup should recover the store");
        assert_eq!(
            store.snapshot().await.profile.name.as_deref(),
            Some("Last known good")
        );
        assert_eq!(
            load_validated_profile_file(&path)
                .expect("primary should be repaired")
                .name
                .as_deref(),
            Some("Last known good")
        );
        let _ = fs::remove_dir_all(path.parent().and_then(Path::parent).unwrap());
    }

    #[tokio::test]
    async fn restores_backup_when_primary_is_missing() {
        let path = temporary_profile_path();
        let first = FullProfile {
            name: Some("Last known good".to_string()),
            ..FullProfile::default()
        };
        persist_profile(&path, &first).expect("first profile should persist");
        let second = FullProfile {
            name: Some("Current".to_string()),
            ..FullProfile::default()
        };
        persist_profile(&path, &second).expect("second profile should persist");
        fs::remove_file(&path).expect("test should remove primary");

        let store = ProfileStore::initialize_at(path.clone())
            .await
            .expect("valid backup should recover the store");
        assert_eq!(
            store.snapshot().await.profile.name.as_deref(),
            Some("Last known good")
        );
        assert!(path.exists());
        let _ = fs::remove_dir_all(path.parent().and_then(Path::parent).unwrap());
    }

    #[test]
    fn recognizes_only_owned_staging_file_names() {
        assert!(is_staging_file_name(
            "1234567890abcdef1234567890abcdef.upload"
        ));
        assert!(!is_staging_file_name("notes.upload"));
        assert!(!is_staging_file_name(
            "1234567890abcdef1234567890abcdef.tmp"
        ));
        assert!(!is_staging_file_name(
            "1234567890abcdef1234567890abcdef0.upload"
        ));
    }

    #[test]
    fn constant_time_comparison_handles_different_lengths() {
        assert!(constant_time_eq(b"same", b"same"));
        assert!(!constant_time_eq(b"same", b"different"));
        assert!(!constant_time_eq(b"same", b"samf"));
    }
}
