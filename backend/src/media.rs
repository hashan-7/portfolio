use axum::Router;
use tower_http::services::ServeDir;

use crate::storage;

pub fn create_media_router() -> Router {
    Router::new().nest_service("/media", ServeDir::new(storage::assets_path()))
}
