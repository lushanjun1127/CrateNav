# 📦 CrateNav - Rust-Powered Bookmark Navigation System

<p align="center">
  <img src="https://shields.io" alt="License">
  <img src="https://shields.io" alt="Backend">
  <img src="https://shields.io" alt="Database">
</p>

<p align="center">
  <strong>A high-performance, full-stack personal bookmark management and navigation hub driven by Rust.</strong><br>
  一款基于纯 Rust 强力驱动、自带轻量后端服务与数据库存储的高性能全栈个人书签导航管理系统。
</p>

---

## ✨ Features / 功能特性

### 🇬🇧 English
- **Rust-Backed Performance**: Built with a lightweight backend (`minimal_server`) written in Rust, ensuring low memory footprint and high concurrency.
- **SQLx & Migrations Included**: Robust data persistence powered by SQLx with built-in database migration schemas for robust environment tracking.
- **Makefile Automation**: Seamless developer workflow orchestrated via simple `make` shortcuts for rapid compilation and local server deployment.

### 🇨🇳 中文
- **Rust 强悍性能**：基于 Rust 语言构建底层的轻量级后端服务 (`minimal_server`)，具备极低的内存占用与闪电级的响应速度。
- **SQLx 数据库持久化**：采用 SQLx 异步安全框架与 SQLite 存储方案，内置规范的数据库迁移 (`migrations`)，数据安全可控。
- **Makefile 自动化构建**：将繁琐的 Cargo 编译、服务启动、数据库检查等复合流水线深度封装为极其高效的 Make 命令行快捷键。

---

## 📂 Repository Structure / 项目结构

```text
├── .sqlx/            # SQLx compile-time offline data cache / SQLx 编译期离线模式数据缓存
├── migrations/       # Database migration tracking files / 数据库迁移结构脚本目录
├── minimal_server/   # Lightweight backend server source code / 轻量级后端服务核心源码
├── src/              # Main logic and build pipeline handlers / Rust 核心业务逻辑实现
├── build.rs          # Rust custom build pipeline script / 自动化编译与资源打包构建脚本
├── Cargo.toml        # Rust package configuration manifest / Rust 项目依赖与包管理配置文件
└── start.bat         # One-click launch script for environment bootstrapping / 一键环境自检与服务运行批处理
```

---

## 🚀 Getting Started / 快速启动

### 🇬🇧 English
1. Clone this repository to your local machine.
2. Configure your environment metrics by replicating the `.env.example` file.
3. Execute standard automated task pipelines via our encapsulated `Makefile` configuration commands.

### 🇨🇳 中文
1. 将本仓库克隆克隆至本地开发环境。
2. 复制 `.env.example` 配置文件并按需调整您本地的运行环境变量参数。
3. 借助内置的 `Makefile` 自动化管线命令，一键进行编译、自检并直接拉起本地导航服务。

---

## 📜 License / 开源协议

This project is licensed under the [MIT License](LICENSE).  
本项目基于 [MIT](LICENSE) 协议完全开源。
