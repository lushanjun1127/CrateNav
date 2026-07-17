@echo off
echo Starting Pintree Rust Server...

REM 检查是否已安装必要的依赖
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Cargo is not installed or not in PATH
    pause
    exit /b 1
)

REM 检查是否有 .env 文件
if not exist ".env" (
    echo Warning: .env file not found, using defaults
    echo Copy .env.example to .env and configure your database settings
)

REM 运行应用
echo Starting server on port 3000...
cargo run

pause