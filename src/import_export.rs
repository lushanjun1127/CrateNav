use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use validator::Validate;
use crate::models::{Collection, Bookmark, Folder, AppError};

#[derive(serde::Deserialize)]
struct BookmarkNode {
    title: String,
    url: Option<String>,
    children: Option<Vec<BookmarkNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkTree {
    pub title: String,
    pub url: Option<String>,
    pub children: Option<Vec<BookmarkTree>>,
    pub folder_name: Option<String>, // To identify if this is inside a folder
}

impl PintreeJson {
    pub async fn import_to_db(&self, pool: &PgPool) -> Result<Collection, AppError> {
        // Create main collection
        let collection_model = crate::models::NewCollection {
            name: self.collections.first().map(|c| c.name.clone()).unwrap_or_else(|| "Imported Collection".to_string()),
            description: self.collections.first().and_then(|c| c.description.clone()),
            icon: self.collections.first().and_then(|c| c.icon.clone()),
            is_public: Some(self.collections.first().map(|c| c.is_public).unwrap_or(true)),
            view_style: Some("grid".to_string()),
            sort_style: Some("alpha".to_string()),
            sort_order: Some(0),
        };

        // Validate the collection data
        collection_model.validate()?;

        let collection = Collection::create(pool, collection_model).await
            .map_err(|e| AppError::Database(e))?;

        // Process each collection's data
        for collection_data in &self.collections {
            // First, create all top-level folders
            let mut folder_mapping = std::collections::HashMap::new();
            for folder_data in &collection_data.folders {
                let new_folder = crate::models::NewFolder {
                    collection_id: collection.id,
                    parent_folder_id: None,
                    name: folder_data.name.clone(),
                    description: folder_data.description.clone(),
                    icon: folder_data.icon.clone(),
                    sort_order: Some(0),
                };
                
                new_folder.validate()?;
                
                let folder = Folder::create(pool, new_folder).await
                    .map_err(|e| AppError::Database(e))?;
                
                folder_mapping.insert(folder_data.name.clone(), folder.id);
                
                // Process subfolders and bookmarks recursively
                Box::pin(Self::import_folder_contents(pool, &folder, &folder_data.subfolders, &folder_data.bookmarks, &collection)).await?;
            }

            // Then create bookmarks not in folders
            for bookmark_data in &collection_data.bookmarks {
                let new_bookmark = crate::models::NewBookmark {
                    collection_id: collection.id,
                    folder_id: None, // Not in a folder
                    title: bookmark_data.title.clone(),
                    url: bookmark_data.url.clone(),
                    description: bookmark_data.description.clone(),
                    icon: bookmark_data.icon.clone(),
                    sort_order: Some(0),
                };
                
                new_bookmark.validate()?;
                
                Bookmark::create(pool, new_bookmark).await
                    .map_err(|e| AppError::Database(e))?;
            }
        }

        Ok(collection)
    }

    async fn import_folder_contents(
        pool: &PgPool,
        parent_folder: &crate::models::Folder,
        subfolders: &[FolderData],
        bookmarks: &[BookmarkData],
        collection: &Collection,
    ) -> Result<(), AppError> {
        // Create bookmarks in this folder
        for bookmark_data in bookmarks {
            let new_bookmark = crate::models::NewBookmark {
                collection_id: collection.id,
                folder_id: Some(parent_folder.id),
                title: bookmark_data.title.clone(),
                url: bookmark_data.url.clone(),
                description: bookmark_data.description.clone(),
                icon: bookmark_data.icon.clone(),
                sort_order: Some(0),
            };
            
            new_bookmark.validate()?;
            
            Bookmark::create(pool, new_bookmark).await
                .map_err(|e| AppError::Database(e))?;
        }

        // Create subfolders recursively
        for subfolder_data in subfolders {
            let new_folder = crate::models::NewFolder {
                collection_id: collection.id,
                parent_folder_id: Some(parent_folder.id),
                name: subfolder_data.name.clone(),
                description: subfolder_data.description.clone(),
                icon: subfolder_data.icon.clone(),
                sort_order: Some(0),
            };
            
            new_folder.validate()?;
            
            let subfolder = Folder::create(pool, new_folder).await
                .map_err(|e| AppError::Database(e))?;
            
            // Recursively process this subfolder's contents
            Box::pin(Self::import_folder_contents(pool, &subfolder, &subfolder_data.subfolders, &subfolder_data.bookmarks, collection)).await?;
        }

        Ok(())
    }
}

pub async fn import_bookmarks_from_chrome_json(pool: &sqlx::PgPool, json_data: &str, collection_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parsed: Value = serde_json::from_str(json_data)?;
    
    if let Some(roots) = parsed.get("roots").and_then(|v| v.as_object()) {
        for (_, root_value) in roots {
            if let Some(children) = root_value.get("children").and_then(|v| v.as_array()) {
                for child in children {
                    process_bookmark_node(pool, child, collection_id, None).await?;
                }
            }
        }
    }
    
    Ok(())
}

async fn process_bookmark_node(
    pool: &sqlx::PgPool,
    node: &Value,
    collection_id: Uuid,
    parent_folder_id: Option<Uuid>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(node_type) = node.get("type").and_then(|v| v.as_str()) {
        match node_type {
            "folder" => {
                // 创建文件夹
                let folder_name = node.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled Folder");
                
                let new_folder = crate::models::NewFolder {
                    collection_id,
                    parent_folder_id,
                    name: folder_name.to_string(),
                    description: node.get("description").and_then(|v| v.as_str()).map(String::from),
                    icon: None,
                    sort_order: Some(0),
                };
                
                new_folder.validate()?;
                
                let folder = Folder::create(pool, new_folder).await
                    .map_err(|e| AppError::Database(e))?;
                
                // 处理子节点
                if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
                    for child in children {
                        process_bookmark_node(pool, child, collection_id, Some(folder.id)).await?;
                    }
                }
            }
            "url" => {
                // 创建书签
                let title = node.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled");
                let url = node.get("url").and_then(|v| v.as_str()).unwrap_or("");
                
                if !url.is_empty() {
                    let new_bookmark = crate::models::NewBookmark {
                        collection_id,
                        folder_id: parent_folder_id,
                        title: title.to_string(),
                        url: url.to_string(),
                        description: None,
                        icon: None,
                        sort_order: Some(0),
                    };
                    
                    new_bookmark.validate()?;
                    
                    Bookmark::create(pool, new_bookmark).await
                        .map_err(|e| AppError::Database(e))?;
                }
            }
            _ => {}
        }
    }
    
    Ok(())
}

pub async fn export_bookmarks_to_chrome_json(pool: &sqlx::PgPool, collection_id: Uuid) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let collection = Collection::find_by_id(pool, collection_id).await
        .map_err(|e| AppError::Database(e))?
        .ok_or(AppError::NotFound)?;
        
    let bookmarks = Bookmark::find_all(pool, Some(collection_id)).await?;
    let folders = Folder::find_by_collection(pool, collection_id).await?;
    
    // 构建文件夹树
    let mut folder_map: HashMap<Uuid, Vec<Bookmark>> = HashMap::new();
    for bookmark in bookmarks {
        let folder_id = bookmark.folder_id.unwrap_or(Uuid::nil()); // 使用nil作为根目录
        folder_map.entry(folder_id).or_insert_with(Vec::new).push(bookmark);
    }
    
    // 创建根节点
    let mut root_children = Vec::new();
    
    // 添加不属于任何文件夹的书签到根目录
    if let Some(root_bookmarks) = folder_map.get(&Uuid::nil()) {
        for bookmark in root_bookmarks {
            root_children.push(create_url_node(bookmark));
        }
    }
    
    // 添加文件夹及其内容
    for folder in folders {
        let folder_node = create_folder_node(&folder, &folder_map);
        root_children.push(folder_node);
    }
    
    let export_data = serde_json::json!({
        "checksum": "0",
        "roots": {
            "bookmark_bar": {
                "children": root_children,
                "name": "Bookmarks Bar",
                "type": "folder"
            },
            "other": {
                "children": [],
                "name": "Other Bookmarks",
                "type": "folder"
            },
            "synced": {
                "children": [],
                "name": "Mobile Bookmarks",
                "type": "folder"
            }
        },
        "version": 1
    });
    
    Ok(serde_json::to_string_pretty(&export_data)?)
}

fn create_url_node(bookmark: &Bookmark) -> serde_json::Value {
    serde_json::json!({
        "date_added": timestamp_to_microseconds(bookmark.created_at.timestamp()),
        "guid": bookmark.id.to_string(),
        "name": bookmark.title,
        "type": "url",
        "url": bookmark.url
    })
}

fn create_folder_node(folder: &Folder, folder_map: &HashMap<Uuid, Vec<Bookmark>>) -> serde_json::Value {
    let mut children = Vec::new();
    
    // 添加属于该文件夹的书签
    if let Some(bookmarks) = folder_map.get(&folder.id) {
        for bookmark in bookmarks {
            children.push(create_url_node(bookmark));
        }
    }
    
    serde_json::json!({
        "children": children,
        "date_added": timestamp_to_microseconds(folder.created_at.timestamp()),
        "date_modified": timestamp_to_microseconds(folder.updated_at.timestamp()),
        "guid": folder.id.to_string(),
        "name": folder.name,
        "type": "folder"
    })
}

fn timestamp_to_microseconds(timestamp: i64) -> u64 {
    // Chrome expects microseconds since epoch
    ((timestamp + 11644473600) * 1_000_000) as u64
}

