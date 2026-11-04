use axum::{Router, routing::get};

use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/api/v1/hello", get(handlers::hello))
}
