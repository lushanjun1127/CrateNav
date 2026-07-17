use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Form,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use std::collections::HashMap;

use crate::models::{
    Collection, CreateCollection, UpdateCollection, Bookmark, CreateBookmark, UpdateBookmark,
    Folder, CreateFolder, UpdateFolder, BookmarkVisit, AdSpace, SearchIndex, SeoSettings,
    AppState, AppError, StatsResponse, BookmarkStats, validate_admin_credentials,
};
use crate::db;

#[derive(Serialize, Deserialize)]
pub struct GetCollectionPath {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCollectionBySlug {
    slug: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBookmarkPath {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetFolderPath {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetVisitPath {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminLogin {
    email: String,
    password: String,
}

pub async fn get_collections(State(state): State<AppState>) -> Result<Json<Vec<Collection>>, AppError> {
    let collections = db::Collection::find_all(&state.pool).await
        .map_err(AppError::Database)?;
    
    Ok(Json(collections))
}

pub async fn create_collection(
    State(state): State<AppState>,
    Form(payload): Form<CreateCollection>
) -> Result<Json<Collection>, AppError> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let collection = db::Collection::create(&state.pool, payload).await
        .map_err(AppError::Database)?;
    
    Ok(Json(collection))
}

pub async fn get_collection_by_id(
    Path(path): Path<GetCollectionPath>,
    State(state): State<AppState>
) -> Result<Json<Collection>, AppError> {
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let collection = db::Collection::find_by_id(&state.pool, id).await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(collection))
}

pub async fn get_collection_by_slug(
    Path(slug): Path<GetCollectionBySlug>,
    State(state): State<AppState>
) -> Result<Json<Collection>, AppError> {
    let collection = db::Collection::find_by_slug(&state.pool, &slug.slug).await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(collection))
}

pub async fn update_collection(
    Path(path): Path<GetCollectionPath>,
    State(state): State<AppState>,
    Form(payload): Form<UpdateCollection>
) -> Result<Json<Collection>, AppError> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let collection = db::Collection::update(
        &state.pool,
        id,
        payload.name,
        payload.description,
        payload.is_public
    ).await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(collection))
}

pub async fn delete_collection(
    Path(path): Path<GetCollectionPath>,
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let rows_affected = db::Collection::delete(&state.pool, id).await
        .map_err(AppError::Database)?;
    
    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::OK)
}

pub async fn get_bookmarks(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>
) -> Result<Json<Vec<Bookmark>>, AppError> {
    let collection_id = params.get("collection_id")
        .and_then(|id_str| Uuid::parse_str(id_str).ok());
    
    let bookmarks = db::Bookmark::find_all(&state.pool, collection_id).await
        .map_err(AppError::Database)?;
    
    Ok(Json(bookmarks))
}

pub async fn create_bookmark(
    State(state): State<AppState>,
    Form(payload): Form<CreateBookmark>
) -> Result<Json<Bookmark>, AppError> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let bookmark = db::Bookmark::create(&state.pool, payload).await
        .map_err(AppError::Database)?;
    
    Ok(Json(bookmark))
}

pub async fn update_bookmark(
    Path(path): Path<GetBookmarkPath>,
    State(state): State<AppState>,
    Form(payload): Form<UpdateBookmark>
) -> Result<Json<Bookmark>, AppError> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let bookmark = db::Bookmark::update(
        &state.pool,
        id,
        payload.title,
        payload.url,
        payload.description,
        payload.icon,
        payload.favicon
    ).await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(bookmark))
}

pub async fn delete_bookmark(
    Path(path): Path<GetBookmarkPath>,
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let rows_affected = db::Bookmark::delete(&state.pool, id).await
        .map_err(AppError::Database)?;
    
    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::OK)
}

pub async fn get_folders(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>
) -> Result<Json<Vec<Folder>>, AppError> {
    let collection_id = params.get("collection_id")
        .and_then(|id_str| Uuid::parse_str(id_str).ok());
    
    let folders = db::Folder::find_all(&state.pool, collection_id).await
        .map_err(AppError::Database)?;
    
    Ok(Json(folders))
}

pub async fn create_folder(
    State(state): State<AppState>,
    Form(payload): Form<CreateFolder>
) -> Result<Json<Folder>, AppError> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let folder = db::Folder::create(&state.pool, payload).await
        .map_err(AppError::Database)?;
    
    Ok(Json(folder))
}

pub async fn update_folder(
    Path(path): Path<GetFolderPath>,
    State(state): State<AppState>,
    Form(payload): Form<UpdateFolder>
) -> Result<Json<Folder>, AppError> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let folder = db::Folder::update(
        &state.pool,
        id,
        payload.name,
        payload.description,
        payload.sort_order
    ).await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(folder))
}

pub async fn delete_folder(
    Path(path): Path<GetFolderPath>,
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    let rows_affected = db::Folder::delete(&state.pool, id).await
        .map_err(AppError::Database)?;
    
    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::OK)
}

pub async fn record_bookmark_visit(
    Path(path): Path<GetVisitPath>,
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&path.id)
        .map_err(|_| AppError::BadRequest("Invalid ID format".to_string()))?;
    
    // 这里可以获取请求的元数据（IP、User-Agent等），但现在使用None作为示例
    let _visit = db::BookmarkVisit::create(&state.pool, id, None, None, None).await
        .map_err(AppError::Database)?;
    
    Ok(StatusCode::OK)
}

pub async fn search_bookmarks(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>
) -> Result<Json<Vec<SearchIndex>>, AppError> {
    let search_term = query.q.unwrap_or_default();
    
    if search_term.is_empty() {
        return Err(AppError::BadRequest("Search query cannot be empty".to_string()));
    }
    
    let results = db::SearchIndex::search(&state.pool, &search_term, 20).await
        .map_err(AppError::Database)?;
    
    Ok(Json(results))
}

pub async fn reindex_search(
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    // 重新索引所有数据
    // 这里只是占位实现
    Ok(StatusCode::OK)
}

pub async fn create_ad_space(
    State(state): State<AppState>,
    Form(payload): Form<HashMap<String, String>>
) -> Result<Json<AdSpace>, AppError> {
    let name = payload.get("name").cloned().unwrap_or_default();
    let position = payload.get("position").cloned().unwrap_or_default();
    let html_content = payload.get("html_content").cloned().unwrap_or_default();
    let is_active = payload.get("is_active").map(|s| s == "true").unwrap_or(true);
    
    let ad_space = db::AdSpace::create(&state.pool, name, position, html_content, is_active).await
        .map_err(AppError::Database)?;
    
    Ok(Json(ad_space))
}

pub async fn get_ad_spaces(
    State(state): State<AppState>
) -> Result<Json<Vec<AdSpace>>, AppError> {
    let ad_spaces = db::AdSpace::find_all(&state.pool).await
        .map_err(AppError::Database)?;
    
    Ok(Json(ad_spaces))
}

pub async fn upsert_seo_settings(
    State(state): State<AppState>,
    Form(payload): Form<HashMap<String, String>>
) -> Result<Json<SeoSettings>, AppError> {
    let collection_id = payload.get("collection_id")
        .and_then(|id_str| Uuid::parse_str(id_str).ok());
    let meta_title = payload.get("meta_title").cloned();
    let meta_description = payload.get("meta_description").cloned();
    let og_title = payload.get("og_title").cloned();
    let og_description = payload.get("og_description").cloned();
    let og_image = payload.get("og_image").cloned();
    let canonical_url = payload.get("canonical_url").cloned();
    let robots = payload.get("robots").cloned();
    
    let seo_setting = db::SeoSettings::upsert(
        &state.pool,
        collection_id,
        meta_title,
        meta_description,
        og_title,
        og_description,
        og_image,
        canonical_url,
        robots
    ).await.map_err(AppError::Database)?;
    
    Ok(Json(seo_setting))
}

pub async fn get_seo_settings(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>
) -> Result<Json<Option<SeoSettings>>, AppError> {
    let collection_id = params.get("collection_id")
        .and_then(|id_str| Uuid::parse_str(id_str).ok());
    
    let seo_setting = db::SeoSettings::find_by_collection_id(&state.pool, collection_id).await
        .map_err(AppError::Database)?;
    
    Ok(Json(seo_setting))
}

pub async fn get_statistics(
    State(state): State<AppState>
) -> Result<Json<StatsResponse>, AppError> {
    use sqlx::Row;
    
    // 获取总数
    let total_collections: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM collections")
        .fetch_one(&*state.pool)
        .await
        .map_err(AppError::Database)?;
    
    let total_bookmarks: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bookmarks")
        .fetch_one(&*state.pool)
        .await
        .map_err(AppError::Database)?;
    
    let total_visits: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bookmark_visits")
        .fetch_one(&*state.pool)
        .await
        .map_err(AppError::Database)?;
    
    // 获取按日期的访问统计
    let visits_by_date: Vec<BookmarkStats> = sqlx::query_as(
        r#"SELECT CAST(DATE(created_at) AS TEXT) as "date!", COUNT(*) as "count!" 
         FROM bookmark_visits 
         GROUP BY DATE(created_at) 
         ORDER BY date DESC LIMIT 30"#
    )
    .fetch_all(&*state.pool)
    .await
    .map_err(AppError::Database)?;
    
    let stats = StatsResponse {
        total_collections: total_collections.0,
        total_bookmarks: total_bookmarks.0,
        total_visits: total_visits.0,
        visits_by_date,
    };
    
    Ok(Json(stats))
}