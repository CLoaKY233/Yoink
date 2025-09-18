use crate::{
    models::models::{AppState, CreateUrlRequest, CreateUrlResponse, UrlRecord, UrlStats},
    utils::utils::generate_short_id,
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use chrono::Utc;
use std::sync::Arc;
use tower_http::trace::OnResponse;
use tracing::{error, info, warn};
use url::Url;
/// Create a new short URL
pub async fn create_short_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUrlRequest>,
) -> impl IntoResponse {
    // Validate the URL
    if Url::parse(&payload.url).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid URL format"
            })),
        )
            .into_response();
    }

    // Generate or use custom ID
    let id = match payload.custom_id {
        // FIX 1: Use `ref custom` to borrow the value instead of moving it.
        Some(ref custom) => {
            if custom.is_empty() || custom.len() > 50 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Custom ID must be between 1 and 50 characters"
                    })),
                )
                    .into_response();
            }
            custom.clone() // Clone the custom ID to use it
        }
        None => generate_short_id(),
    };

    // Check if custom ID already exists
    if payload.custom_id.is_some() {
        let existing: Result<Option<UrlRecord>, _> = state.db.select(("urls", &id)).await;
        match existing {
            Ok(Some(_)) => {
                return (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({
                        "error": "Custom ID already exists"
                    })),
                )
                    .into_response();
            }
            Err(e) => {
                error!("Database error checking custom ID: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "Database error"
                    })),
                )
                    .into_response();
            }
            _ => {}
        }
    }

    let url_record = UrlRecord::new(id.clone(), payload.url.clone());

    /// Insert into database
    let result: Result<Option<UrlRecord>, _> =
        state.db.create(("urls", &id)).content(url_record).await;

    match result {
        Ok(Some(record)) => {
            info!("Created short URL: {} -> {}", id, payload.url);
            let response = CreateUrlResponse {
                short_url: format!("{}/{}", state.base_url, id),
                original_url: record.original_url,
                id: record.short_id,
                created_at: record.created_at,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Ok(None) => {
            error!("Failed to create short URL: No Record Returned");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error":"Failed to create short URL"
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to create short URL: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create short URL"
                })),
            )
                .into_response()
        }
    }
}

pub async fn redirect_url(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let current_record: Result<Option<UrlRecord>, _> = state.db.select(("urls", &id)).await;
    match current_record {
        Ok(Some(mut record)) => {
            // Update click count and last accessed
            record.click_count += 1;
            record.last_accessed = Some(Utc::now());

            // Update the record in the database
            // FIX 3: Move `record` into `.content()` instead of passing a reference.
            let _update_result: Result<Option<UrlRecord>, _> = state
                .db
                .update(("urls", &id))
                .content(record.clone()) // Clone record to pass ownership and still use it later
                .await;

            info!("Redirecting {} to {}", id, record.original_url);
            Redirect::permanent(&record.original_url).into_response()
        }
        Ok(None) => {
            warn!("Short URL not found: {}", id);
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Short URL not found"
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Internal server error"
                })),
            )
                .into_response()
        }
    }
}

pub async fn get_url_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result: Result<Option<UrlRecord>, _> = state.db.select(("urls", &id)).await;
    match result {
        Ok(Some(record)) => {
            let stats = UrlStats {
                id: record.short_id.clone(),
                original_url: record.original_url,
                short_url: format!("{}/{}", state.base_url, record.short_id),
                click_count: record.click_count,
                created_at: record.created_at,
                last_accessed: record.last_accessed,
            };
            (StatusCode::OK, Json(stats)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Short URL not found"
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Internal server error"
                })),
            )
                .into_response()
        }
    }
}
