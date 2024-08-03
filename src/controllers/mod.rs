use sqlx::{Pool, Postgres, Row};
use warp::reply::Json;
use futures::TryStreamExt;
use serde_json::json;


pub async fn get_users(pool: Pool<Postgres>) -> Result<Json, warp::Rejection> {
    let mut rows = sqlx::query("SELECT * FROM users;").fetch(&pool);
    let mut users = Vec::new();
    while let Some(row) = rows.try_next().await.expect("Failed to get rows") {
        let id: i32 = row.try_get("id").expect("Failed to try_get id");
        let email:String= row.try_get("email").expect("Failed to try_get email");
        let password:String = row.try_get("password").expect("Failed to try_get password");
        let user = json!({
            "id": id,
            "email": email,
            "password": password
        });
        users.push(user);
    }
    Ok(warp::reply::json(&users))
}
