use sqlx::PgPool;

pub async fn get_db_pool() -> PgPool {
    // implementation stub
    PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap()
}

pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!().run(pool).await.expect("Failed to run migrations");
}