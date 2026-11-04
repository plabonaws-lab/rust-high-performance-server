use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ServerInfo {
    service: &'static str,
    message: &'static str,
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

#[derive(Serialize)]
pub struct HelloResponse {
    message: &'static str,
}

pub async fn root() -> Json<ServerInfo> {
    Json(ServerInfo {
        service: "rust-high-performance-server",
        message: "Welcome to our Rust HTTP server!",
    })
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Rust!",
    })
}
