use std::{env, net::SocketAddr};

pub mod admin;
pub mod auth;
pub mod media;
pub mod portfolio_bot;
pub mod profile;
pub mod routes;
pub mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "7860".to_string())
        .parse::<u16>()?;

    let app = routes::create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Server starting on http://{}", addr);
    println!("📖 Swagger UI available on http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
