@echo off
echo Starting Pintree Rust Server...

REM 检查是否已安装必要的依赖
echo Checking prerequisites...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Cargo is not installed or not in PATH
    pause
    exit /b 1
)

sqlx --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Warning: sqlx-cli is not installed. Installing...
    cargo install sqlx-cli --version 0.9.0
    if %errorlevel% neq 0 (
        echo Error: Failed to install sqlx-cli
        pause
        exit /b 1
    )
)

REM 检查是否有 .env 文件
if not exist ".env" (
    echo Error: .env file not found
    echo Please copy .env.example to .env and configure your database settings
    pause
    exit /b 1
)

echo Checking database connectivity...
cargo run --bin check-db 2>nul
if %errorlevel% neq 0 (
    echo Database is not accessible or migrations haven't been run yet.
    echo Attempting to run migrations...
    sqlx migrate run
    if %errorlevel% neq 0 (
        echo Error: Failed to run database migrations
        echo Please ensure PostgreSQL is running and credentials are correct in .env
        pause
        exit /b 1
    )
    echo Database migrations completed successfully!
)

echo Starting server on port 3000...
cargo run

pause