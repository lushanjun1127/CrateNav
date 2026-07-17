use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCollection {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCollection {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bookmark {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub folder_id: Option<Uuid>,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub favicon: Option<String>,
    pub visit_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateBookmark {
    pub collection_id: Uuid,
    pub folder_id: Option<Uuid>,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(url)]
    pub url: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub icon: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateBookmark {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(url)]
    pub url: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub icon: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFolder {
    pub collection_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[serde(default)]
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFolder {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BookmarkVisit {
    pub id: Uuid,
    pub bookmark_id: Uuid,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdSpace {
    pub id: Uuid,
    pub name: String,
    pub position: String, // header, sidebar, footer, content
    pub html_content: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SearchIndex {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub bookmark_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SeoSettings {
    pub id: Uuid,
    pub collection_id: Option<Uuid>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub canonical_url: Option<String>,
    pub robots: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// AppState不应该被序列化，因为PgPool不能被序列化
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: std::sync::Arc<sqlx::PgPool>,
}

// 添加缺失的统计响应结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsResponse {
    pub total_collections: i64,
    pub total_bookmarks: i64,
    pub total_visits: i64,
    pub visits_by_date: Vec<BookmarkStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct BookmarkStats {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::Validation(format!("{}", errors))
    }
}

// 管理员验证函数
pub fn validate_admin_credentials(email: &str, password: &str) -> bool {
    use std::env;
    
    let admin_email = env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());
    
    email == admin_email && password == admin_password
}