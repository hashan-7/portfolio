#[cfg(test)]
use crate::profile::FullProfile;
use crate::storage::{self, ProfileStore};
use anyhow::{Context, Result};
use std::{env, sync::Arc};
use tokio::sync::Mutex;

const DEFAULT_MAX_MEDIA_STORAGE_BYTES: u64 = 512 * 1024 * 1024;
const MIN_MEDIA_STORAGE_BYTES: u64 = 25 * 1024 * 1024;
const MAX_MEDIA_STORAGE_BYTES: u64 = 10 * 1024 * 1024 * 1024;

#[derive(Clone)]
pub struct AppState {
    pub profiles: ProfileStore,
    pub upload_lock: Arc<Mutex<()>>,
    pub max_media_storage_bytes: u64,
}

impl AppState {
    pub async fn initialize() -> Result<Self> {
        storage::cleanup_upload_staging()
            .await
            .context("Failed to clean interrupted uploads")?;

        let max_media_storage_bytes = env::var("MAX_MEDIA_STORAGE_BYTES")
            .ok()
            .map(|value| {
                value
                    .parse::<u64>()
                    .context("MAX_MEDIA_STORAGE_BYTES must be an integer")
            })
            .transpose()?
            .unwrap_or(DEFAULT_MAX_MEDIA_STORAGE_BYTES);

        if !(MIN_MEDIA_STORAGE_BYTES..=MAX_MEDIA_STORAGE_BYTES).contains(&max_media_storage_bytes) {
            anyhow::bail!(
                "MAX_MEDIA_STORAGE_BYTES must be between {MIN_MEDIA_STORAGE_BYTES} and {MAX_MEDIA_STORAGE_BYTES}"
            );
        }

        Ok(Self {
            profiles: ProfileStore::initialize().await?,
            upload_lock: Arc::new(Mutex::new(())),
            max_media_storage_bytes,
        })
    }

    #[cfg(test)]
    pub fn for_test(profile: FullProfile) -> Self {
        Self {
            profiles: ProfileStore::for_test(profile),
            upload_lock: Arc::new(Mutex::new(())),
            max_media_storage_bytes: DEFAULT_MAX_MEDIA_STORAGE_BYTES,
        }
    }
}
