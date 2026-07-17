use axum::{
    extract::State,
    routing::{get, post, patch, delete},
    Router,
};
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;

mod models;
mod db;
mod handlers;
mod import_export;
mod app;
use crate::models::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从.env文件加载环境变量
    dotenvy::dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 数据库连接
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = db::init_pool().await?;
    let shared_state = AppState {
        pool: std::sync::Arc::new(pool),
    };

    // CORS配置
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 创建带状态的API路由
    let app = Router::new()
        // API 路由
        .route("/api/collections", get(handlers::get_collections).post(handlers::create_collection))
        .route("/api/collections/:id", get(handlers::get_collection_by_id).patch(handlers::update_collection).delete(handlers::delete_collection))
        .route("/api/collections/slug/:slug", get(handlers::get_collection_by_slug))
        .route("/api/bookmarks", get(handlers::get_bookmarks).post(handlers::create_bookmark))
        .route("/api/bookmarks/:id", patch(handlers::update_bookmark).delete(handlers::delete_bookmark))
        .route("/api/folders", get(handlers::get_folders).post(handlers::create_folder))
        .route("/api/folders/:id", patch(handlers::update_folder).delete(handlers::delete_folder))
        .route("/api/import", post(handlers::import_bookmarks))
        .route("/api/export/:id", get(handlers::export_bookmarks))
        .route("/api/visit/:id", post(handlers::record_bookmark_visit))  // 专业版功能
        .route("/api/search", get(handlers::search_bookmarks))  // 专业版功能
        .route("/api/reindex", post(handlers::reindex_search))  // 专业版功能
        .route("/api/ad-spaces", post(handlers::create_ad_space).get(handlers::get_ad_spaces))  // 专业版功能
        .route("/api/seo-settings", post(handlers::upsert_seo_settings).get(handlers::get_seo_settings))  // 专业版功能
        .route("/api/stats", get(handlers::get_statistics))  // 专业版功能
        .layer(cors_layer)
        .with_state(shared_state);

    // 服务器配置
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u16>().unwrap_or(3000);
    let addr = SocketAddr::from((host.parse::<std::net::IpAddr>().unwrap(), port));

    println!("Server is running on http://{}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    ).await?;

    Ok(())
}