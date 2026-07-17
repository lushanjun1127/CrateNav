use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载数据库URL
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("正在连接到数据库...");
    println!("数据库URL: {}", database_url);

    // 创建数据库连接池
    let pool = PgPool::connect(&database_url).await?;
    
    println!("✅ 数据库连接成功！");

    // 执行一个简单的查询来测试连接
    let result = sqlx::query("SELECT version()")
        .fetch_one(&pool)
        .await?;
    
    let version: &str = result.try_get("version")?;
    println!("📊 PostgreSQL 版本: {}", version);

    // 尝试查询 collections 表（我们的应用数据表）
    let count_result = sqlx::query("SELECT COUNT(*) as count FROM collections")
        .fetch_one(&pool)
        .await;
        
    match count_result {
        Ok(row) => {
            let count: i64 = row.try_get("count")?;
            println!("📁 collections 表中的记录数: {}", count);
        }
        Err(_) => {
            println!("📁 collections 表可能不存在或为空");
        }
    }

    // 关闭连接池
    pool.close().await;
    println!("🔒 数据库连接已关闭");

    Ok(())
}