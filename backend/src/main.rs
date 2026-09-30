use std::{env, net::SocketAddr};

use tracing_subscriber::EnvFilter;

pub mod admin;
pub mod auth;
pub mod error;
pub mod media;
pub mod portfolio_bot;
pub mod profile;
pub mod rate_limit;
pub mod routes;
pub mod state;
pub mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    initialize_tracing()?;

    auth::initialize_from_env()?;
    let rate_limit_config = rate_limit::initialize_from_env()?;

    let port = env::var("PORT")
        .unwrap_or_else(|_| "7860".to_string())
        .parse::<u16>()?;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;
    let app = routes::create_router().await?;

    tracing::info!(
        %address,
        trust_proxy_headers = rate_limit_config.trust_proxy_headers,
        "Portfolio backend is ready"
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

fn initialize_tracing() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("backend=info,tower_http=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init()
        .map_err(|error| anyhow::anyhow!("Failed to initialize tracing: {error}"))?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "Failed to install Ctrl+C shutdown handler");
        }
    };

    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let terminate = async {
            match signal(SignalKind::terminate()) {
                Ok(mut stream) => {
                    stream.recv().await;
                }
                Err(error) => {
                    tracing::error!(%error, "Failed to install SIGTERM shutdown handler");
                    std::future::pending::<()>().await;
                }
            }
        };

        tokio::select! {
            () = ctrl_c => {},
            () = terminate => {},
        }
    }

    #[cfg(not(unix))]
    ctrl_c.await;

    tracing::info!("Shutdown signal received; draining active requests");
}
