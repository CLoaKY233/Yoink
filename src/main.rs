use anyhow::Result;
use axum::{
    Json, Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
mod config;
mod database;
mod handlers;
mod models;
mod utils;
use config::Config;
use handlers::{create_short_url, get_url_stats, redirect_url};
use models::AppState;
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    info!("Starting URL Shortener service...");
    // Load configuration
    let config = Config::from_env()?;
    // Setup database
    let db = database::setup_database(
        &config.database_url,
        &config.surreal_username,
        &config.surreal_password,
    )
    .await?;
    // Create shared application state
    let app_state = Arc::new(AppState {
        db,
        base_url: config.base_url.clone(),
    });
    // Build our application with routes
    let app = Router::new()
        .route("/", post(create_short_url))
        // FIX: Changed route from "/:id" to "/{id}"
        .route("/{id}", get(redirect_url))
        // FIX: Changed route from "/api/stats/:id" to "/api/stats/{id}"
        .route("/api/stats/{id}", get(get_url_stats))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server_address).await?;
    info!("Server running on {}", config.server_address);
    axum::serve(listener, app).await?;
    Ok(())
}
async fn health_check() -> axum::response::Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "url-shortener",
        "database": "surrealdb"
    }))
}
