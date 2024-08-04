use futures::TryStreamExt;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use warp::reply::Json;

pub async fn get_users(pool: Pool<Postgres>) -> Result<Json, warp::Rejection> {
    let mut rows = sqlx::query("SELECT * FROM users;").fetch(&pool);
    let mut users = Vec::new();
    while let Some(row) = rows.try_next().await.expect("Failed to get rows") {
        let id: i32 = row.try_get("id").expect("Failed to try_get id");
        let email: String = row.try_get("email").expect("Failed to try_get email");
        let user = json!({
            "id": id,
            "email": email,
        });
        users.push(user);
    }
    Ok(warp::reply::json(&users))
}

pub async fn create_user(
    pool: Pool<Postgres>,
    email: String,
    password: String,
) -> Result<Json, warp::Rejection> {
    let row =
        sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email;")
            .bind(email)
            .bind(password)
            .fetch_one(&pool)
            .await
            .expect("Failed inserting user");
    let id: i32 = row.try_get("id").expect("Failed to try_get id");
    let email: String = row.try_get("email").expect("Failed to try_get email");
    let user = json!({
        "id": id,
        "email": email,
    });
    Ok(warp::reply::json(&user))
}
