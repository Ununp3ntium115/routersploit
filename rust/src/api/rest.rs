// REST API implementation using axum

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use anyhow::Result;

// API routes will be added here

pub async fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}
