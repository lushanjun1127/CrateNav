use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

use crate::models::{
    Collection, CreateCollection, UpdateCollection, 
    Bookmark, CreateBookmark, UpdateBookmark, 
    Folder, CreateFolder, UpdateFolder, 
    BookmarkVisit, AdSpace, SearchIndex, SeoSettings
};

impl Collection {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Collection>, sqlx::Error> {
        sqlx::query_as!(
            Collection,
            r#"SELECT 
                id, name, slug, description, is_public, 
                created_at, updated_at 
               FROM collections 
               ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Collection>, sqlx::Error> {
        sqlx::query_as!(
            Collection,
            r#"SELECT 
                id, name, slug, description, is_public, 
                created_at, updated_at 
               FROM collections 
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Collection>, sqlx::Error> {
        sqlx::query_as!(
            Collection,
            r#"SELECT 
                id, name, slug, description, is_public, 
                created_at, updated_at 
               FROM collections 
               WHERE slug = $1"#,
            slug
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, data: CreateCollection) -> Result<Collection, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let collection = sqlx::query_as!(
            Collection,
            r#"INSERT INTO collections (id, name, slug, description, is_public, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, name, slug, description, is_public, created_at, updated_at"#,
            id,
            data.name,
            slugify(&data.name),
            data.description,
            data.is_public.unwrap_or(true),
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(collection)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        is_public: Option<bool>,
    ) -> Result<Option<Collection>, sqlx::Error> {
        let collection = sqlx::query_as!(
            Collection,
            r#"UPDATE collections 
               SET name = COALESCE($2, name),
                   slug = CASE WHEN $2 IS NOT NULL THEN $2 || '-' || $1::TEXT ELSE slug END, -- 更新名称时更新slug
                   description = COALESCE($3, description),
                   is_public = COALESCE($4, is_public),
                   updated_at = $5
               WHERE id = $1
               RETURNING id, name, slug, description, is_public, created_at, updated_at"#,
            id,
            name,
            description,
            is_public,
            Utc::now()
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(collection)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM collections WHERE id = $1"#,
            id
        )
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}

impl Bookmark {
    pub async fn find_all(pool: &PgPool, collection_id: Option<Uuid>) -> Result<Vec<Bookmark>, sqlx::Error> {
        if let Some(col_id) = collection_id {
            sqlx::query_as!(
                Bookmark,
                r#"SELECT 
                    id, collection_id, folder_id, title, url, description, 
                    icon, favicon, visit_count, created_at, updated_at 
                   FROM bookmarks 
                   WHERE collection_id = $1
                   ORDER BY created_at DESC"#,
                col_id
            )
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as!(
                Bookmark,
                r#"SELECT 
                    id, collection_id, folder_id, title, url, description, 
                    icon, favicon, visit_count, created_at, updated_at 
                   FROM bookmarks 
                   ORDER BY created_at DESC"#
            )
            .fetch_all(pool)
            .await
        }
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Bookmark>, sqlx::Error> {
        sqlx::query_as!(
            Bookmark,
            r#"SELECT 
                id, collection_id, folder_id, title, url, description, 
                icon, favicon, visit_count, created_at, updated_at 
               FROM bookmarks 
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, data: CreateBookmark) -> Result<Bookmark, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let bookmark = sqlx::query_as!(
            Bookmark,
            r#"INSERT INTO bookmarks (id, collection_id, folder_id, title, url, description, icon, favicon, visit_count, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING id, collection_id, folder_id, title, url, description, icon, favicon, visit_count, created_at, updated_at"#,
            id,
            data.collection_id,
            data.folder_id,
            data.title,
            data.url,
            data.description,
            data.icon,
            data.favicon,
            0_i32, // 默认访问次数为0
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(bookmark)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        title: Option<String>,
        url: Option<String>,
        description: Option<String>,
        icon: Option<String>,
        favicon: Option<String>,
    ) -> Result<Option<Bookmark>, sqlx::Error> {
        let bookmark = sqlx::query_as!(
            Bookmark,
            r#"UPDATE bookmarks 
               SET title = COALESCE($2, title),
                   url = COALESCE($3, url),
                   description = COALESCE($4, description),
                   icon = COALESCE($5, icon),
                   favicon = COALESCE($6, favicon),
                   updated_at = $7
               WHERE id = $1
               RETURNING id, collection_id, folder_id, title, url, description, icon, favicon, visit_count, created_at, updated_at"#,
            id,
            title,
            url,
            description,
            icon,
            favicon,
            Utc::now()
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(bookmark)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM bookmarks WHERE id = $1"#,
            id
        )
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}

impl Folder {
    pub async fn find_all(pool: &PgPool, collection_id: Option<Uuid>) -> Result<Vec<Folder>, sqlx::Error> {
        if let Some(col_id) = collection_id {
            sqlx::query_as!(
                Folder,
                r#"SELECT 
                    id, collection_id, parent_id, name, description, 
                    sort_order, created_at, updated_at 
                   FROM folders 
                   WHERE collection_id = $1
                   ORDER BY sort_order, created_at DESC"#,
                col_id
            )
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as!(
                Folder,
                r#"SELECT 
                    id, collection_id, parent_id, name, description, 
                    sort_order, created_at, updated_at 
                   FROM folders 
                   ORDER BY sort_order, created_at DESC"#
            )
            .fetch_all(pool)
            .await
        }
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Folder>, sqlx::Error> {
        sqlx::query_as!(
            Folder,
            r#"SELECT 
                id, collection_id, parent_id, name, description, 
                sort_order, created_at, updated_at 
               FROM folders 
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_collection(pool: &PgPool, collection_id: Uuid) -> Result<Vec<Folder>, sqlx::Error> {
        sqlx::query_as!(
            Folder,
            r#"SELECT 
                id, collection_id, parent_id, name, description, 
                sort_order, created_at, updated_at 
               FROM folders 
               WHERE collection_id = $1
               ORDER BY sort_order, created_at DESC"#,
            collection_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(pool: &PgPool, data: CreateFolder) -> Result<Folder, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let folder = sqlx::query_as!(
            Folder,
            r#"INSERT INTO folders (id, collection_id, parent_id, name, description, sort_order, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               RETURNING id, collection_id, parent_id, name, description, sort_order, created_at, updated_at"#,
            id,
            data.collection_id,
            data.parent_id,
            data.name,
            data.description,
            data.sort_order.unwrap_or(0),
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(folder)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        sort_order: Option<i32>,
    ) -> Result<Option<Folder>, sqlx::Error> {
        let folder = sqlx::query_as!(
            Folder,
            r#"UPDATE folders 
               SET name = COALESCE($2, name),
                   description = COALESCE($3, description),
                   sort_order = COALESCE($4, sort_order),
                   updated_at = $5
               WHERE id = $1
               RETURNING id, collection_id, parent_id, name, description, sort_order, created_at, updated_at"#,
            id,
            name,
            description,
            sort_order,
            Utc::now()
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(folder)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM folders WHERE id = $1"#,
            id
        )
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}

impl BookmarkVisit {
    pub async fn create(
        pool: &PgPool,
        bookmark_id: Uuid,
        ip_address: Option<String>,
        user_agent: Option<String>,
        referer: Option<String>,
    ) -> Result<BookmarkVisit, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let visit = sqlx::query_as!(
            BookmarkVisit,
            r#"INSERT INTO bookmark_visits (id, bookmark_id, ip_address, user_agent, referer, created_at)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING id, bookmark_id, ip_address, user_agent, referer, created_at"#,
            id,
            bookmark_id,
            ip_address,
            user_agent,
            referer,
            now
        )
        .fetch_one(pool)
        .await?;
        
        // 增加书签的访问次数
        sqlx::query!(
            r#"UPDATE bookmarks SET visit_count = visit_count + 1 WHERE id = $1"#,
            bookmark_id
        )
        .execute(pool)
        .await?;
        
        Ok(visit)
    }
}

impl AdSpace {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<AdSpace>, sqlx::Error> {
        sqlx::query_as!(
            AdSpace,
            r#"SELECT 
                id, name, position, html_content, is_active, 
                created_at, updated_at 
               FROM ad_spaces 
               ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<AdSpace>, sqlx::Error> {
        sqlx::query_as!(
            AdSpace,
            r#"SELECT 
                id, name, position, html_content, is_active, 
                created_at, updated_at 
               FROM ad_spaces 
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        name: String,
        position: String,
        html_content: String,
        is_active: bool,
    ) -> Result<AdSpace, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let ad_space = sqlx::query_as!(
            AdSpace,
            r#"INSERT INTO ad_spaces (id, name, position, html_content, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, name, position, html_content, is_active, created_at, updated_at"#,
            id,
            name,
            position,
            html_content,
            is_active,
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(ad_space)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<String>,
        position: Option<String>,
        html_content: Option<String>,
        is_active: Option<bool>,
    ) -> Result<Option<AdSpace>, sqlx::Error> {
        let ad_space = sqlx::query_as!(
            AdSpace,
            r#"UPDATE ad_spaces 
               SET name = COALESCE($2, name),
                   position = COALESCE($3, position),
                   html_content = COALESCE($4, html_content),
                   is_active = COALESCE($5, is_active),
                   updated_at = $6
               WHERE id = $1
               RETURNING id, name, position, html_content, is_active, created_at, updated_at"#,
            id,
            name,
            position,
            html_content,
            is_active,
            Utc::now()
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(ad_space)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM ad_spaces WHERE id = $1"#,
            id
        )
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}

impl SearchIndex {
    pub async fn search(pool: &PgPool, query: &str, limit: i32) -> Result<Vec<SearchIndex>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);
        
        sqlx::query_as!(
            SearchIndex,
            r#"SELECT 
                id, collection_id, bookmark_id, folder_id, title, content, url, 
                created_at, updated_at 
               FROM search_indices 
               WHERE title ILIKE $1 OR content ILIKE $1
               ORDER BY created_at DESC
               LIMIT $2"#,
            search_pattern,
            limit
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create_index(
        pool: &PgPool,
        collection_id: Uuid,
        bookmark_id: Option<Uuid>,
        folder_id: Option<Uuid>,
        title: String,
        content: String,
        url: String,
    ) -> Result<SearchIndex, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let index = sqlx::query_as!(
            SearchIndex,
            r#"INSERT INTO search_indices (id, collection_id, bookmark_id, folder_id, title, content, url, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               RETURNING id, collection_id, bookmark_id, folder_id, title, content, url, created_at, updated_at"#,
            id,
            collection_id,
            bookmark_id,
            folder_id,
            title,
            content,
            url,
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(index)
    }
}

impl SeoSettings {
    pub async fn find_by_collection_id(pool: &PgPool, collection_id: Option<Uuid>) -> Result<Option<SeoSettings>, sqlx::Error> {
        if let Some(col_id) = collection_id {
            sqlx::query_as!(
                SeoSettings,
                r#"SELECT 
                    id, collection_id, meta_title, meta_description, 
                    og_title, og_description, og_image, canonical_url, 
                    robots, created_at, updated_at 
                   FROM seo_settings 
                   WHERE collection_id = $1"#,
                col_id
            )
            .fetch_optional(pool)
            .await
        } else {
            sqlx::query_as!(
                SeoSettings,
                r#"SELECT 
                    id, collection_id, meta_title, meta_description, 
                    og_title, og_description, og_image, canonical_url, 
                    robots, created_at, updated_at 
                   FROM seo_settings 
                   WHERE collection_id IS NULL"#  // 全局SEO设置
            )
            .fetch_optional(pool)
            .await
        }
    }

    pub async fn upsert(
        pool: &PgPool,
        collection_id: Option<Uuid>,
        meta_title: Option<String>,
        meta_description: Option<String>,
        og_title: Option<String>,
        og_description: Option<String>,
        og_image: Option<String>,
        canonical_url: Option<String>,
        robots: Option<String>,
    ) -> Result<SeoSettings, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        // 先尝试更新现有记录
        if let Some(col_id) = collection_id {
            // 尝试更新特定集合的SEO设置
            if let Some(existing) = sqlx::query_as!(
                SeoSettings,
                r#"SELECT 
                    id, collection_id, meta_title, meta_description, 
                    og_title, og_description, og_image, canonical_url, 
                    robots, created_at, updated_at 
                   FROM seo_settings 
                   WHERE collection_id = $1"#,
                col_id
            )
            .fetch_optional(pool)
            .await?
            {
                // 如果存在，则更新
                return sqlx::query_as!(
                    SeoSettings,
                    r#"UPDATE seo_settings 
                       SET meta_title = COALESCE($2, meta_title),
                           meta_description = COALESCE($3, meta_description),
                           og_title = COALESCE($4, og_title),
                           og_description = COALESCE($5, og_description),
                           og_image = COALESCE($6, og_image),
                           canonical_url = COALESCE($7, canonical_url),
                           robots = COALESCE($8, robots),
                           updated_at = $9
                       WHERE id = $1
                       RETURNING id, collection_id, meta_title, meta_description, 
                                 og_title, og_description, og_image, canonical_url, 
                                 robots, created_at, updated_at"#,
                    existing.id,
                    meta_title,
                    meta_description,
                    og_title,
                    og_description,
                    og_image,
                    canonical_url,
                    robots,
                    now
                )
                .fetch_one(pool)
                .await;
            } else {
                // 如果不存在，则插入新记录
                return sqlx::query_as!(
                    SeoSettings,
                    r#"INSERT INTO seo_settings (id, collection_id, meta_title, meta_description, 
                                                 og_title, og_description, og_image, canonical_url, 
                                                 robots, created_at, updated_at)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                       RETURNING id, collection_id, meta_title, meta_description, 
                                 og_title, og_description, og_image, canonical_url, 
                                 robots, created_at, updated_at"#,
                    id,
                    collection_id,
                    meta_title,
                    meta_description,
                    og_title,
                    og_description,
                    og_image,
                    canonical_url,
                    robots,
                    now,
                    now
                )
                .fetch_one(pool)
                .await;
            }
        } else {
            // 处理全局SEO设置
            if let Some(existing) = sqlx::query_as!(
                SeoSettings,
                r#"SELECT 
                    id, collection_id, meta_title, meta_description, 
                    og_title, og_description, og_image, canonical_url, 
                    robots, created_at, updated_at 
                   FROM seo_settings 
                   WHERE collection_id IS NULL"#  // 全局SEO设置
            )
            .fetch_optional(pool)
            .await?
            {
                // 如果存在，则更新
                return sqlx::query_as!(
                    SeoSettings,
                    r#"UPDATE seo_settings 
                       SET meta_title = COALESCE($2, meta_title),
                           meta_description = COALESCE($3, meta_description),
                           og_title = COALESCE($4, og_title),
                           og_description = COALESCE($5, og_description),
                           og_image = COALESCE($6, og_image),
                           canonical_url = COALESCE($7, canonical_url),
                           robots = COALESCE($8, robots),
                           updated_at = $9
                       WHERE id = $1 AND collection_id IS NULL
                       RETURNING id, collection_id, meta_title, meta_description, 
                                 og_title, og_description, og_image, canonical_url, 
                                 robots, created_at, updated_at"#,
                    existing.id,
                    meta_title,
                    meta_description,
                    og_title,
                    og_description,
                    og_image,
                    canonical_url,
                    robots,
                    now
                )
                .fetch_one(pool)
                .await;
            } else {
                // 如果不存在，则插入新记录
                return sqlx::query_as!(
                    SeoSettings,
                    r#"INSERT INTO seo_settings (id, collection_id, meta_title, meta_description, 
                                                 og_title, og_description, og_image, canonical_url, 
                                                 robots, created_at, updated_at)
                       VALUES ($1, NULL, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                       RETURNING id, collection_id, meta_title, meta_description, 
                                 og_title, og_description, og_image, canonical_url, 
                                 robots, created_at, updated_at"#,
                    id,
                    meta_title,
                    meta_description,
                    og_title,
                    og_description,
                    og_image,
                    canonical_url,
                    robots,
                    now,
                    now
                )
                .fetch_one(pool)
                .await;
            }
        }
    }
}

// 辅助函数
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}