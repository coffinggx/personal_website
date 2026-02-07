use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn get_db_pool() -> Result<PgPool, sqlx::Error> {
    dotenvy::dotenv().ok();
    let uri = env::var("DATABASE_URL").expect("Database connection string must be provided");
    let pool = PgPoolOptions::new().max_connections(5).connect(&uri).await;
    return pool;
}
