use axum::{
    routing::{get, post},
    Json, Router,
};
use chatbot_ml::chatbot::client::AiClient;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

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
    paths(health_check, chat_handler),
    components(schemas(ChatRequest, ChatResponse)),
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

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(chat_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}