use crate::helpers::hash_password;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_password =
        env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD env variable not set");
    let database_url = format!(
        "postgres://postgres:{}@localhost/postgres",
        database_password
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

pub async fn setup_database(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            email VARCHAR(50) UNIQUE NOT NULL,
            password VARCHAR(225) NOT NULL
        )
        ",
    )
    .execute(&pool)
    .await?;

    let email = "benjaminpla.dev@gmail.com";
    let password = env::var("ADMIN_PASSWORD").expect("Missing \"ADMIN_PASSWORD\" env variable");
    let hashed_password = hash_password(&password);
    sqlx::query(
        "INSERT INTO users (email, password) VALUES ($1, $2) ON CONFLICT (email) DO NOTHING;",
    )
    .bind(email)
    .bind(hashed_password)
    .execute(&pool)
    .await?;

    Ok(())
}
