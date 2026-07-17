-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(50) DEFAULT 'user',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 收藏夹/集合表
CREATE TABLE IF NOT EXISTS collections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon VARCHAR(255),
    is_public BOOLEAN DEFAULT true,
    view_style VARCHAR(50) DEFAULT 'grid',
    sort_style VARCHAR(50) DEFAULT 'alpha',
    sort_order INTEGER DEFAULT 0,
    slug VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 文件夹表
CREATE TABLE IF NOT EXISTS folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID REFERENCES collections(id) ON DELETE CASCADE,
    parent_folder_id UUID REFERENCES folders(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon VARCHAR(255),
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 书签表
CREATE TABLE IF NOT EXISTS bookmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    folder_id UUID REFERENCES folders(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    description TEXT,
    icon VARCHAR(255),
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 书签访问统计表（专业版功能）
CREATE TABLE IF NOT EXISTS bookmark_visits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bookmark_id UUID NOT NULL REFERENCES bookmarks(id) ON DELETE CASCADE,
    user_agent TEXT,
    ip_address INET,
    referrer TEXT,
    visited_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 广告位表（专业版功能）
CREATE TABLE IF NOT EXISTS ad_spaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID REFERENCES collections(id) ON DELETE CASCADE, -- NULL表示全局广告位
    position VARCHAR(50) NOT NULL, -- 'header', 'sidebar', 'footer', 'inline'
    ad_code TEXT NOT NULL,
    is_enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 搜索索引表（专业版功能）
CREATE TABLE IF NOT EXISTS search_index (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bookmark_id UUID NOT NULL UNIQUE REFERENCES bookmarks(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    url TEXT,
    tags JSONB, -- 存储标签的JSON数组
    indexed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- SEO设置表（专业版功能）
CREATE TABLE IF NOT EXISTS seo_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID UNIQUE REFERENCES collections(id) ON DELETE CASCADE, -- NULL表示全局设置
    meta_title VARCHAR(255),
    meta_description TEXT,
    meta_keywords TEXT,
    og_title VARCHAR(255),
    og_description TEXT,
    og_image TEXT,
    canonical_url TEXT,
    structured_data JSONB, -- JSON-LD格式的结构化数据
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 创建全文搜索索引（用于AI搜索功能）
CREATE INDEX IF NOT EXISTS idx_search_index_fulltext ON search_index USING GIN(to_tsvector('english', title || ' ' || COALESCE(description, '') || ' ' || COALESCE(url, '')));

-- 创建访问统计索引
CREATE INDEX IF NOT EXISTS idx_bookmark_visits_bookmark_id ON bookmark_visits(bookmark_id);
CREATE INDEX IF NOT EXISTS idx_bookmark_visits_visited_at ON bookmark_visits(visited_at);

-- 创建广告位索引
CREATE INDEX IF NOT EXISTS idx_ad_spaces_collection_id ON ad_spaces(collection_id);
CREATE INDEX IF NOT EXISTS idx_ad_spaces_position ON ad_spaces(position);

-- 创建SEO设置索引
CREATE INDEX IF NOT EXISTS idx_seo_settings_collection_id ON seo_settings(collection_id);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_collections_is_public ON collections(is_public);
CREATE INDEX IF NOT EXISTS idx_collections_slug ON collections(slug);
CREATE INDEX IF NOT EXISTS idx_bookmarks_collection_id ON bookmarks(collection_id);
CREATE INDEX IF NOT EXISTS idx_bookmarks_folder_id ON bookmarks(folder_id);
CREATE INDEX IF NOT EXISTS idx_folders_collection_id ON folders(collection_id);
CREATE INDEX IF NOT EXISTS idx_folders_parent_folder_id ON folders(parent_folder_id);