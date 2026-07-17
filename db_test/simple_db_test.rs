use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载数据库URL
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            // 如果环境变量未设置，则使用 .env 文件中的值
            println!("正在从 .env 文件加载数据库URL...");
            dotenv::dotenv().ok();
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in .env file")
        });

    println!("正在连接到数据库...");
    println!("数据库URL: {}", database_url.replace("npg_3D9yCvHPmsaL", "***"));

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

    // 尝试查询其他表
    let tables = ["bookmarks", "folders", "users", "seo_settings", "ad_spaces", "search_indexes"];
    for table in &tables {
        let count_result = sqlx::query(&format!("SELECT COUNT(*) as count FROM {}", table))
            .fetch_one(&pool)
            .await;
            
        match count_result {
            Ok(row) => {
                let count: i64 = row.try_get("count")?;
                println!("📁 {} 表中的记录数: {}", table, count);
            }
            Err(_) => {
                println!("📁 {} 表可能不存在或为空", table);
            }
        }
    }

    // 测试插入和删除一条测试记录（不会真正执行，只是测试权限）
    println!("🔍 测试数据库写入权限...");
    match sqlx::query("CREATE TEMP TABLE test_table (id SERIAL PRIMARY KEY, name VARCHAR(50));").execute(&pool).await {
        Ok(_) => {
            println!("✅ 可以创建临时表（写入权限正常）");
            // 清理临时表
            let _ = sqlx::query("DROP TABLE test_table;").execute(&pool).await;
        },
        Err(e) => {
            println!("⚠️ 创建临时表失败: {}", e);
        }
    }

    // 关闭连接池
    pool.close().await;
    println!("🔒 数据库连接已关闭");

    Ok(())
}