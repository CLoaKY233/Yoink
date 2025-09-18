use anyhow::Result;
use axum::{
    Json, Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use url_shortener::config::config::Config;

use url_shortener::database::database;
use url_shortener::handlers::handlers::{create_short_url, get_url_stats, redirect_url};
use url_shortener::models::models::AppState;
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    info!("Starting URL Shortener service...");
    let config = Config::from_env()?;
    let db = database::setup_database(
        &config.database_url,
        &config.surreal_ns,
        &config.surreal_db,
        &config.surreal_username,
        &config.surreal_password,
    )
    .await?;

    let app_state = Arc::new(AppState {
        db,
        base_url: config.base_url.clone(),
    });

    let app = Router::new()
        .route("/", post(create_short_url))
        .route("/{id}", get(redirect_url))
        .route("/api/stats/{id}", get(get_url_stats))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(&config.server_address).await?;
    info!("Server running on {}", config.server_address);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> axum::response::Json<serde_json::Value> {
    Json(serde_json::json!({
        "status":"healthy",
        "service":"url_shortener",
        "database":"surrealdb"
    }))
}
