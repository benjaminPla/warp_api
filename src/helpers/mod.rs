use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub id: i32,
}

pub enum TokenValidationError {
    Expired,
    Invalid,
    Other,
}

pub fn hash_password(password: &str) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error on `hash_password`")
        .to_string();
    password_hash
}

pub fn verify_password(hashed_password: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_password).expect("Error parsing hashed password");
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

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

pub fn create_token(user: User) -> Result<String, ()> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_|())?;
    let jwt_secret_as_bytes = jwt_secret.as_bytes();
    let claims = Claims {
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        user,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_secret_as_bytes),
    )
    .map_err(|_| ())
}

pub fn verify_token(token: String) -> Result<TokenData<Claims>, TokenValidationError> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| TokenValidationError::Other)?;
    let jwt_secret_as_bytes = jwt_secret.as_bytes();
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret_as_bytes),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => Ok(token_data),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err(TokenValidationError::Expired),
            ErrorKind::InvalidToken => Err(TokenValidationError::Invalid),
            _ => Err(TokenValidationError::Other),
        },
    }
}
