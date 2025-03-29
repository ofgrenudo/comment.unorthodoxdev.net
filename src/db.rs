use dotenv::dotenv;
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};
use std::env;

pub async fn create_pool() -> Result<Pool<MySql>, sqlx::Error> {
    dotenv().ok(); // Load .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .connect(&database_url)
        .await?;

    Ok(pool)
}
