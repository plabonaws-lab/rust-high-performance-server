mod handlers;
mod routes;

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app = routes::create_router();

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind TCP listener");

    tracing::info!("Rust HTTP server listening on {}", address);

    axum::serve(listener, app).await.expect("Server failed");
}
