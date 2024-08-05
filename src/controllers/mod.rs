use crate::errors::ServerError;
use crate::helpers::hash_password;
use crate::routes::CreateOrUpdateUserRequest;
use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};
use warp::{Rejection, Reply};

#[derive(Serialize)]
struct DefaultUserResponse {
    id: i32,
    email: String,
}

pub async fn get_users(pool: Pool<Postgres>) -> Result<impl Reply, Rejection> {
    let mut rows = sqlx::query("SELECT * FROM users;").fetch(&pool);
    let mut users = Vec::new();
    while let Some(row) = rows
        .try_next()
        .await
        .map_err(|_| ServerError::InternalServerError)?
    {
        let id: i32 = row
            .try_get("id")
            .map_err(|_| ServerError::InternalServerError)?;
        let email: String = row
            .try_get("email")
            .map_err(|_| ServerError::InternalServerError)?;
        let user = DefaultUserResponse { id, email };
        users.push(user);
    }
    Ok(warp::reply::json(&users))
}

pub async fn create_user(
    pool: Pool<Postgres>,
    body: CreateOrUpdateUserRequest,
) -> Result<impl Reply, Rejection> {
    let email = body.email;
    let password = body.password;
    let hashed_password = hash_password(&password);
    let row =
        sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email;")
            .bind(email)
            .bind(hashed_password)
            .fetch_one(&pool)
            .await;
    match row {
        Ok(row) => {
            let id: i32 = row
                .try_get("id")
                .map_err(|_| ServerError::InternalServerError)?;
            let email: String = row
                .try_get("email")
                .map_err(|_| ServerError::InternalServerError)?;
            let user = DefaultUserResponse { id, email };
            Ok(warp::reply::json(&user))
        }
        Err(_) => Err(ServerError::InternalServerError)?,
    }
}

pub async fn update_user(
    pool: Pool<Postgres>,
    id: i32,
    body: CreateOrUpdateUserRequest,
) -> Result<impl Reply, Rejection> {
    let email = body.email;
    let password = body.password;
    let hashed_password = hash_password(&password);
    let row = sqlx::query(
        "UPDATE users SET email = $1, password = $2 WHERE id = $3 RETURNING id, email;",
    )
    .bind(email)
    .bind(hashed_password)
    .bind(id)
    .fetch_one(&pool)
    .await;
    match row {
        Ok(row) => {
            let id: i32 = row
                .try_get("id")
                .map_err(|_| ServerError::InternalServerError)?;
            let email: String = row
                .try_get("email")
                .map_err(|_| ServerError::InternalServerError)?;
            let user = DefaultUserResponse { id, email };
            Ok(warp::reply::json(&user))
        }
        Err(sqlx::Error::RowNotFound) => Err(ServerError::NotFound)?,
        Err(_) => Err(ServerError::InternalServerError)?,
    }
}

pub async fn delete_user(pool: Pool<Postgres>, id: i32) -> Result<impl Reply, Rejection> {
    let row = sqlx::query("DELETE FROM users WHERE id = $1 RETURNING id, email;")
        .bind(id)
        .fetch_one(&pool)
        .await;
    match row {
        Ok(row) => {
            let id: i32 = row
                .try_get("id")
                .map_err(|_| ServerError::InternalServerError)?;
            let email: String = row
                .try_get("email")
                .map_err(|_| ServerError::InternalServerError)?;
            let user = DefaultUserResponse { id, email };
            Ok(warp::reply::json(&user))
        }
        Err(sqlx::Error::RowNotFound) => Err(ServerError::NotFound)?,
        Err(_) => Err(ServerError::InternalServerError)?,
    }
}
