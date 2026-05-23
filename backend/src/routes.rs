use axum::{
    Json, Router,
    http::StatusCode,
    middleware,
    routing::{get, post},
};
use chatbot_ml::chatbot::client::{AiClient, ChatMessage as MlChatMessage};
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
    media,
    profile::{Certificate, ChatScope, Education, PublicProfile, PublicProject, SocialLinks},
    safety, storage,
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
    #[serde(default)]
    pub scope: ChatScope,
}

#[derive(Serialize, ToSchema)]
pub struct ChatResponse {
    pub reply: String,
    pub scope: ChatScope,
}

#[derive(OpenApi)]
#[openapi(
    paths(health_check, chat_handler, profile_handler),
    components(schemas(
        ChatRequest,
        ChatResponse,
        ChatMessage,
        ChatScope,
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
    responses((status = 200, description = "Chatbot response successfully generated", body = ChatResponse))
)]
async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    let scope = payload.scope;

    let history = payload
        .history
        .into_iter()
        .map(|message| MlChatMessage {
            role: message.role,
            content: message.content,
        })
        .collect::<Vec<_>>();

    if let Some(reply) = safety::get_safety_reply(&history) {
        return Json(ChatResponse { reply, scope });
    }

    let profile_context = match storage::load_chatbot_context_json(scope) {
        Ok(profile_context) => profile_context,
        Err(error) => {
            return Json(ChatResponse {
                reply: format!("[Error] Failed to load portfolio context: {}", error),
                scope,
            });
        }
    };

    let reply = AiClient::get_reply(
        &history,
        &profile_context,
        scope.as_prompt_label(),
        scope.as_display_label(),
    )
    .await;

    Json(ChatResponse { reply, scope })
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
