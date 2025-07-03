use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn establish_connection_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5) // Jumlah koneksi maksimum dalam pool
        .min_connections(1) // Jumlah koneksi minimum
        .acquire_timeout(Duration::from_secs(3)) // Timeout untuk mendapatkan koneksi
        .connect(&database_url)
        .await
}
