# Pintree

Convert your browser bookmarks into a beautiful, customizable, and deployable directory website. No coding required.

Pintree is an open-source navigation website generator that transforms your browser bookmarks into a stunning, customizable, and deployable directory site. Built for personal use, teams, and communities, it offers a modern alternative to traditional bookmark managers with powerful features for organization and sharing.

## ✨ Features

- 📑 **Import & Export**: Support for browser bookmark exports (HTML, JSON, Netscape format)
- 📁 **Hierarchical Management**: Organize bookmarks into categorized collections with folder structures
- 🎨 **Theming & SEO**: Customize styles, favicons, meta tags, and Open Graph properties
- 🔍 **Global Search**: Client-side and server-side indexing for fast discovery
- 🔒 **Access Control**: Public, login-required, and private visibility options
- 📊 **Statistics Dashboard**: Track visits, popular bookmarks, and traffic sources (PRO)

## 🛠️ Tech Stack

- **Frontend**: Leptos (Rust + WASM)
- **Backend**: Axum (Rust async web framework)
- **Database**: PostgreSQL with SQLx
- **Build Tool**: cargo-leptos

## 📋 依赖要求

- Rust 1.70+
- PostgreSQL 12+

## 快速开始

### 环境要求

- Rust (稳定版)
- PostgreSQL 数据库
- `sqlx-cli` 工具 (`cargo install sqlx-cli`)

### 环境变量配置

项目需要以下三个核心环境变量：

1. **DATABASE_URL** (必需) - PostgreSQL数据库连接字符串
   ```
   DATABASE_URL="postgresql://postgres:password@localhost/pintree"
   ```

2. **ADMIN_EMAIL** (必需) - 管理员邮箱
   ```
   ADMIN_EMAIL="admin@example.com"
   ```

3. **ADMIN_PASSWORD** (必需) - 管理员密码
   ```
   ADMIN_PASSWORD="admin123"
   ```

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Pintree-io/pintree.git
   cd pintree
   ```

2. **Install dependencies**
   ```bash
   # Install cargo-leptos build tool
   cargo install cargo-leptos
   ```

3. **Set up environment variables**
   ```bash
   # Copy the example environment file
   cp .env.example .env
   
   # Edit .env with your database connection and admin credentials
   ```

4. 确保 PostgreSQL 服务正在运行

#### 在 Windows 上启动 PostgreSQL:
- 如果使用 PostgreSQL 官方发行版:
  ```bash
  # 通常在 "开始" 菜单中启动 "pgAdmin 4" 或运行服务
  # 或使用命令行启动服务 (需要管理员权限):
  net start postgresql-x64-14  # 版本号可能不同
  ```

#### 创建数据库:
```bash
# 使用 psql 命令行工具 (确保在 PATH 中)
psql -U postgres -c "CREATE DATABASE pintree;"
psql -U postgres -c "CREATE USER postgres WITH PASSWORD 'password';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE pintree TO postgres;"
```

4. **Run database migrations**
   ```bash
   # Execute the migration script
   psql -d "$DATABASE_URL" -f migrations/001_init.sql
   ```

5. **Start the development server**
   ```bash
   # Run the application
   cargo leptos serve
   ```

6. **Access the application**
   - Frontend: [http://localhost:3000](http://localhost:3000)
   - Admin Panel: [http://localhost:3000/admin](http://localhost:3000/admin)

或者使用提供的启动脚本：
```bash
./start.bat  # Windows
```

## 数据库迁移

项目使用 SQLx 进行数据库迁移。迁移文件位于 `migrations/` 目录。

## 部署

### 生产环境构建
```bash
cargo build --release
```

### Docker 部署（可选）
```dockerfile
FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates postgresql-client
COPY --from=builder /usr/src/app/target/release/pintree /usr/local/bin/pintree
CMD ["pintree"]
```

### 环境变量配置

确保在生产环境中配置以下环境变量：
- `DATABASE_URL` - PostgreSQL 数据库连接字符串
- `PORT` - 监听端口 (默认: 8080)
- `LEPTOS_SITE_ADDR` - 绑定地址 (默认: 0.0.0.0:8080)
- `LEPTOS_SITE_NAME` - 站点名称
- `LEPTOS_DEBUG` - 调试模式 (生产环境应设置为 false)

服务器将在 `http://localhost:3000` 上运行。

## 📁 Project Structure

```
src/
├── app.rs          # Leptos root component and routing
├── models.rs       # Data models with Serde and SQLx derives
├── db.rs           # Database access layer with SQLx queries
├── handlers.rs     # Axum request handlers
├── import_export.rs # Bookmark import/export logic
└── main.rs         # Axum application entry point
```

## API 接口

### 收藏夹管理
- `GET /api/collections` - 获取所有收藏夹
- `POST /api/collections` - 创建收藏夹
- `GET /api/collections/:id` - 获取指定收藏夹
- `PUT /api/collections/:id` - 更新收藏夹
- `DELETE /api/collections/:id` - 删除收藏夹

### 书签管理
- `GET /api/bookmarks?collection_id=:id` - 获取书签
- `POST /api/bookmarks` - 创建书签
- `PUT /api/bookmarks/:id` - 更新书签
- `DELETE /api/bookmarks/:id` - 删除书签

### 文件夹管理
- `GET /api/folders?collection_id=:id` - 获取文件夹
- `POST /api/folders` - 创建文件夹
- `PUT /api/folders/:id` - 更新文件夹
- `DELETE /api/folders/:id` - 删除文件夹

### 导入导出
- `POST /api/import` - 导入书签
- `GET /api/export/:id` - 导出书签

### 专业版功能
- `POST /api/visit/:id` - 记录书签访问
- `GET /api/search?q=:query` - 搜索书签
- `POST /api/reindex` - 重建搜索索引
- `POST /api/ad-spaces` - 创建广告位
- `GET /api/ad-spaces` - 获取广告位
- `POST /api/seo-settings` - 设置SEO配置
- `GET /api/seo-settings` - 获取SEO配置
- `GET /api/stats` - 获取统计数据

## 🤝 Contributing

We welcome contributions! Here's how you can help:

- 🐛 **Bug Reports**: Found a bug? [Open an issue](https://github.com/Pintree-io/pintree/issues) with detailed reproduction steps.
- ✨ **Feature Requests**: Have an idea? [Submit a feature request](https://github.com/Pintree-io/pintree/issues) with use cases and implementation thoughts.
- 🛠️ **Pull Requests**: Want to contribute code? Fork the repo, create a branch, and submit a PR with a clear description of your changes.

## 📄 License

MIT © [Pintree](https://github.com/Pintree-io/pintree)

## 🙏 Acknowledgments

- Inspired by the need for better bookmark management solutions
- Built with the amazing Rust ecosystem

<div align="center">

[English](./README.md) | [简体中文](./README-zh.md)

  <h3>Pintree - Turn Your Browser Bookmarks into a Directory Website</h3>
  <p>Create and monetize your own directory website from browser bookmarks in minutes.</p>
</div>

## 🔗 Links

- [Pintree Official Website](https://pintree.io)
- [Demo](https://demo.pintree.io)
- [Documentation](https://docs.pintree.io)
- [Changelog](https://docs.pintree.io/en/changelog)

## ✨ Features

### Basic Version (Free)
- 📑 Unlimited Import/Export Bookmarks
- 📁 Bookmark Management
- 🎨 Basic Theme Customization
- 🔍 Bookmark Search

### [Professional Version (PRO)](https://www.pintree.io/#pricing)
- 📑 All Basic Version Features
- 📚 Multiple Collection Switching
- 🔒 Private Collections
- 📢 Multiple Ad Space Configuration
- 🤖 AI Search
- 🎯 Professional SEO Optimization
- 📊 Detailed Access Statistics
- 💻 Priority Technical Support
- 🕒 Lifetime Access
- 🔄 Free Lifetime Updates

## 🛠️ Tech Stack

- **Frontend&Backend**: Next.js
- **Deployment**: Vercel
- **Database**: PostgreSQL

## 👥 Community

- Twitter: [@pintree_io](https://twitter.com/pintree_io)
- GitHub: [github.com/Pintree-io](https://github.com/Pintree-io)
- Discord: [Join Community](https://discord.gg/gJTrkHFg)
- Email: feedback@pintree.io

## ❤️ Contributing

We welcome Issues and Pull Requests to help improve this documentation.