use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::env;
use tokio;

// 简单的数据模型
#[derive(Serialize, Deserialize, FromRow, Clone)]
struct Collection {
    id: i32,
    name: String,
    slug: String,
    description: Option<String>,
    is_public: bool,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// 获取所有集合的处理器
async fn get_collections(State(state): State<AppState>) -> Result<Json<Vec<Collection>>, StatusCode> {
    match sqlx::query_as("SELECT id, name, slug, description, is_public FROM collections ORDER BY id")
        .fetch_all(&state.db)
        .await
    {
        Ok(collections) => Ok(Json(collections)),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 获取单个集合的处理器
async fn get_collection(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Collection>, StatusCode> {
    match sqlx::query_as("SELECT id, name, slug, description, is_public FROM collections WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(collection) => Ok(Json(collection)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    println!("Connecting to database...");
    
    // 创建数据库连接池
    let db_pool = PgPool::connect(&database_url).await?;
    
    // 运行一次简单的查询验证连接
    let result = sqlx::query("SELECT version()")
        .fetch_one(&db_pool)
        .await?;
    
    let version: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(&db_pool)
        .await?;
    println!("✅ Database connected. Version: {}", version.0);

    // 创建应用状态
    let app_state = AppState { db: db_pool };

    // 构建我们的应用程序路由
    let app = Router::new()
        .route("/api/collections", get(get_collections))
        .route("/api/collections/:id", get(get_collection))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://0.0.0.0:3000");
    println!("📚 Try:");
    println!("   GET  http://0.0.0.0:3000/api/collections");
    println!("   GET  http://0.0.0.0:3000/api/collections/1");

    axum::serve(listener, app).await?;

    Ok(())
}