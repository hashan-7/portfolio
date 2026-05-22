use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chatbot_ml::chatbot::client::AiClient;
use serde::{Deserialize, Serialize};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tech_stack: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SocialLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProfileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<Project>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<Certificate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<SocialLinks>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ChatRequest {
    pub message: String,
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
        ProfileResponse,
        Project,
        Certificate,
        SocialLinks,
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
    let reply = AiClient::get_reply(&payload.message).await;

    Json(ChatResponse { reply })
}

#[utoipa::path(
    get,
    path = "/api/profile",
    tag = "Portfolio API",
    responses(
        (status = 200, description = "Public profile data retrieved successfully", body = ProfileResponse),
        (status = 404, description = "Profile data not found", body = ErrorResponse),
        (status = 500, description = "Failed to parse profile data", body = ErrorResponse)
    )
)]
async fn profile_handler() -> Result<Json<ProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let profile_json = env::var("PORTFOLIO_PROFILE_JSON").map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Profile data is not configured.".to_string(),
            }),
        )
    })?;

    serde_json::from_str::<ProfileResponse>(&profile_json).map(Json).map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to parse profile data: {}", error),
            }),
        )
    })
}

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .route("/api/profile", get(profile_handler))
        .route("/api/chat", post(chat_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
}