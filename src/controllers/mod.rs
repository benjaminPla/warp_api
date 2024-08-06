use crate::errors::{ServerError, UnauthorizedTypes};
use crate::helpers::{create_token, hash_password, verify_password, User};
use crate::routes::UserRequest;
use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};
use warp::{Rejection, Reply};

#[derive(Serialize)]
struct DefaultUserResponse {
    id: i32,
    email: String,
}

#[derive(Serialize)]
struct AuthenticateResponse {
    token: String,
}

pub async fn authenticate(
    pool: Pool<Postgres>,
    body: UserRequest,
) -> Result<impl Reply, Rejection> {
    let email = body.email;
    let password = body.password;
    let row = sqlx::query("SELECT id, email, password FROM users WHERE email = $1;")
        .bind(email)
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
            let hashed_password: String = row
                .try_get("password")
                .map_err(|_| ServerError::InternalServerError)?;
            let user = User { email, id };
            match verify_password(&hashed_password, &password) {
                true => match create_token(user) {
                    Ok(token) => Ok(warp::reply::json(&AuthenticateResponse { token })),
                    Err(_) => Err(ServerError::InternalServerError)?,
                },
                false => Err(ServerError::Unauthorized(UnauthorizedTypes::Default))?,
            }
        }
        Err(sqlx::Error::RowNotFound) => Err(ServerError::NotFound)?,
        Err(_) => Err(ServerError::InternalServerError)?,
    }
}

pub async fn get_users(pool: Pool<Postgres>, _: ()) -> Result<impl Reply, Rejection> {
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
    _: (),
    body: UserRequest,
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
    _: (),
    id: i32,
    body: UserRequest,
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

pub async fn delete_user(pool: Pool<Postgres>, _: (), id: i32) -> Result<impl Reply, Rejection> {
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
