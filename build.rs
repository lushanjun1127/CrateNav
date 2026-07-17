fn main() {
    // 设置SQLX_OFFLINE环境变量为true，以启用离线模式
    std::env::set_var("SQLX_OFFLINE", "true");
}