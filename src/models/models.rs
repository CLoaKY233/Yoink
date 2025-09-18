use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub custom_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub short_url: String,
    pub original_url: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UrlStats {
    pub id: String,
    pub original_url: String,
    pub short_url: String,
    pub click_count: i64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UrlRecord {
    pub id: Option<surrealdb::sql::Thing>,
    pub short_id: String,
    pub original_url: String,
    pub click_count: i64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

impl UrlRecord {
    pub fn new(short_id: String, original_url: String) -> Self {
        Self {
            id: None,
            short_id,
            original_url,
            click_count: 0,
            created_at: Utc::now(),
            last_accessed: None,
        }
    }
}
