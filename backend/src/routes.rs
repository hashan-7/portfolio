use axum::{
    Json, Router,
    http::StatusCode,
    middleware,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    admin::{
        get_admin_profile_handler, login_handler, update_admin_profile_handler,
        upload_media_handler, verify_admin_handler,
    },
    auth::auth_middleware,
    media, portfolio_bot,
    profile::{Certificate, Education, PublicProfile, PublicProject, SocialLinks},
    storage,
};

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ChatRequest {
    pub history: Vec<ChatMessage>,
}

#[derive(Serialize, ToSchema)]
pub struct ChatResponse {
    pub reply: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(health_check, chat_handler, profile_handler),
    components(schemas(
        ChatRequest,
        ChatResponse,
        ChatMessage,
        PublicProfile,
        PublicProject,
        Certificate,
        SocialLinks,
        Education,
        ErrorResponse
    )),
    tags((name = "Portfolio API", description = "Backend routes for AI Portfolio"))
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/health",
    tag = "Portfolio API",
    responses((status = 200, description = "Server is running smoothly", body = String))
)]
async fn health_check() -> &'static str {
    "OK"
}

#[utoipa::path(
    post,
    path = "/api/chat",
    tag = "Portfolio API",
    request_body = ChatRequest,
    responses((status = 200, description = "Portfolio assistant response generated", body = ChatResponse))
)]
async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    let latest_user_message = payload
        .history
        .iter()
        .rev()
        .find(|message| message.role.trim() == "user")
        .map(|message| message.content.trim())
        .unwrap_or("");

    let recent_context = payload
        .history
        .iter()
        .rev()
        .take(12)
        .map(|message| format!("{}: {}", message.role.trim(), message.content.trim()))
        .collect::<Vec<_>>();

    let full_profile = match storage::load_profile() {
        Ok(full_profile) => full_profile,
        Err(error) => {
            return Json(ChatResponse {
                reply: format!("[Error] Failed to load portfolio data: {}", error),
            });
        }
    };

    let reply =
        portfolio_bot::get_portfolio_reply(&full_profile, latest_user_message, &recent_context);

    Json(ChatResponse { reply })
}

#[utoipa::path(
    get,
    path = "/api/profile",
    tag = "Portfolio API",
    responses(
        (status = 200, description = "Public profile data retrieved successfully", body = PublicProfile),
        (status = 500, description = "Failed to load profile data", body = ErrorResponse)
    )
)]
async fn profile_handler() -> Result<Json<PublicProfile>, (StatusCode, Json<ErrorResponse>)> {
    storage::load_profile()
        .map(|full_profile| Json(full_profile.into()))
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to load profile data: {}", error),
                }),
            )
        })
}

fn admin_routes() -> Router {
    let protected_routes = Router::new()
        .route("/verify", get(verify_admin_handler))
        .route(
            "/profile",
            get(get_admin_profile_handler).put(update_admin_profile_handler),
        )
        .route("/media/upload", post(upload_media_handler))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/login", post(login_handler))
        .merge(protected_routes)
}

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let frontend_serve_dir =
        ServeDir::new("frontend/dist").fallback(ServeFile::new("frontend/dist/index.html"));

    Router::new()
        .route("/health", get(health_check))
        .route("/api/profile", get(profile_handler))
        .route("/api/chat", post(chat_handler))
        .nest("/api/admin", admin_routes())
        .merge(media::create_media_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .fallback_service(frontend_serve_dir)
        .layer(cors)
}
