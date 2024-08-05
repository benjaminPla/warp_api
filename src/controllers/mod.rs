use crate::errors::InternalServerError;
use crate::routes::CreateOrUpdateUserRequest;
use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};
use warp::reply::Json;

#[derive(Serialize)]
struct DefaultUserResponse {
    id: i32,
    email: String,
}

pub async fn get_users(pool: Pool<Postgres>) -> Result<Json, warp::Rejection> {
    let mut rows = sqlx::query("SELECT * FROM users;").fetch(&pool);
    let mut users = Vec::new();
    while let Some(row) = rows
        .try_next()
        .await
        .map_err(|_| warp::reject::custom(InternalServerError))?
    {
        let id: i32 = row
            .try_get("id")
            .map_err(|_| warp::reject::custom(InternalServerError))?;
        let email: String = row
            .try_get("email")
            .map_err(|_| warp::reject::custom(InternalServerError))?;
        let user = DefaultUserResponse { id, email };
        users.push(user);
    }
    Ok(warp::reply::json(&users))
}

pub async fn create_user(
    pool: Pool<Postgres>,
    body: CreateOrUpdateUserRequest,
) -> Result<Json, warp::Rejection> {
    let email = body.email;
    let password = body.password;
    let row =
        sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email;")
            .bind(email)
            .bind(password)
            .fetch_one(&pool)
            .await
            .map_err(|_| warp::reject::custom(InternalServerError))?;
    let id: i32 = row
        .try_get("id")
        .map_err(|_| warp::reject::custom(InternalServerError))?;
    let email: String = row
        .try_get("email")
        .map_err(|_| warp::reject::custom(InternalServerError))?;
    let user = DefaultUserResponse { id, email };
    Ok(warp::reply::json(&user))
}

pub async fn update_user(
    pool: Pool<Postgres>,
    id: i32,
    body: CreateOrUpdateUserRequest,
) -> Result<Json, warp::Rejection> {
    let email = body.email;
    let password = body.password;
    let row = sqlx::query(
        "UPDATE users SET email = $1, password = $2 WHERE id = $3 RETURNING id, email;",
    )
    .bind(email)
    .bind(password)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| warp::reject::custom(InternalServerError))?;
    let id: i32 = row
        .try_get("id")
        .map_err(|_| warp::reject::custom(InternalServerError))?;
    let email: String = row
        .try_get("email")
        .map_err(|_| warp::reject::custom(InternalServerError))?;
    let user = DefaultUserResponse { id, email };
    Ok(warp::reply::json(&user))
}

pub async fn delete_user(
    pool: Pool<Postgres>,
    id: i32,
) -> Result<Json, warp::Rejection> {
    let row = sqlx::query(
        "DELETE FROM users WHERE id = $1 RETURNING id, email;",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| warp::reject::custom(InternalServerError))?;
    let id: i32 = row
        .try_get("id")
        .map_err(|_| warp::reject::custom(InternalServerError))?;
    let email: String = row
        .try_get("email")
        .map_err(|_| warp::reject::custom(InternalServerError))?;
    let user = DefaultUserResponse { id, email };
    Ok(warp::reply::json(&user))
}
