# Pintree

<div align="center">

[English](./README.md) | [简体中文](./README-zh.md)

  <h3>Pintree - 将您的浏览器书签转换为导航网站</h3>
  <p>在几分钟内从浏览器书签创建一个导航网站。</p>
</div>

## 🔗 链接

- [Pintree 官方网站](https://pintree.io/zh)
- [演示](https://demo.pintree.io)
- [文档](https://docs.pintree.io)
- [更新日志](https://docs.pintree.io/zh/changelog)

## ✨ 功能

- 📑 **导入与导出**：支持浏览器书签导出（HTML、JSON、Netscape 格式）
- 📁 **分层管理**：将书签组织成分类集合和文件夹结构
- 🎨 **主题与 SEO**：自定义样式、网站图标、元标签和开放图谱属性
- 🔍 **全局搜索**：客户端和服务端索引以快速发现
- 🔒 **访问控制**：公共、需要登录和私有可见性选项
- 📊 **统计仪表板**：跟踪访问量、热门书签和流量来源（专业版）

## 🛠️ 技术栈

- **前端**: Leptos (Rust + WASM)
- **后端**: Axum (Rust 异步网络框架)
- **数据库**: PostgreSQL 与 SQLx
- **构建工具**: cargo-leptos

## 👥 社区

- 推特: [@pintree_io](https://twitter.com/pintree_io)
- GitHub: [github.com/Pintree-io](https://github.com/Pintree-io)
- Discord: [加入社区](https://discord.gg/gJTrkHFg)
- 邮箱: feedback@pintree.io

## 🚀 快速开始

### 先决条件

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [PostgreSQL](https://www.postgresql.org/download/) (12+)

### 安装

1. **克隆仓库**
   ```bash
   git clone https://github.com/Pintree-io/pintree.git
   cd pintree
   ```

2. **安装依赖**
   ```bash
   # 安装 cargo-leptos 构建工具
   cargo install cargo-leptos
   ```

3. **设置环境变量**
   ```bash
   # 复制示例环境文件
   cp .env.example .env
   
   # 编辑 .env 文件，填入数据库连接和管理员凭据
   ```

4. **运行数据库迁移**
   ```bash
   # 执行迁移脚本
   psql -d "$DATABASE_URL" -f migrations/001_init.sql
   ```

5. **启动开发服务器**
   ```bash
   # 运行应用程序
   cargo leptos serve
   ```

6. **访问应用程序**
   - 前端: [http://localhost:3000](http://localhost:3000)
   - 管理面板: [http://localhost:3000/admin](http://localhost:3000/admin)

## 📁 项目结构

```
src/
├── app.rs          # Leptos 根组件和路由
├── models.rs       # 带有 Serde 和 SQLx 衍生的数据模型
├── db.rs           # 带有 SQLx 查询的数据库访问层
├── handlers.rs     # Axum 请求处理器
├── import_export.rs # 书签导入/导出逻辑
└── main.rs         # Axum 应用程序入口点
```

## 🤝 贡献

我们欢迎贡献！以下是可以帮助的方式：

- 🐛 **错误报告**：发现问题？[提交问题](https://github.com/Pintree-io/pintree/issues)，附上详细的重现步骤。
- ✨ **功能请求**：有想法？[提交功能请求](https://github.com/Pintree-io/pintree/issues)，说明使用场景和实现思路。
- 🛠️ **拉取请求**：想贡献代码？Fork 仓库，创建分支，提交 PR 并附上清晰的更改说明。

## 📄 许可证

MIT © [Pintree](https://github.com/Pintree-io/pintree)

## 🙏 致谢

- 受改善书签管理解决方案需求的启发
- 基于令人惊叹的 Rust 生态系统构建