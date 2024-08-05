use crate::controllers::{create_user, get_users};
use crate::errors::handle_errors;
use crate::middlewares::with_db;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use warp::Filter;

#[derive(Deserialize)]
struct CreateUserRequest {
    email: String,
    password: String,
}

pub fn create_routes(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let status = warp::path::end().and(warp::get()).map(|| "Up");

    let get_users_route = warp::path("get_users")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(get_users)
        .recover(handle_errors);

    let create_user_route = warp::path("create_user")
        .and(warp::post())
        .and(with_db(pool.clone()))
        .and(warp::body::json::<CreateUserRequest>())
        .and_then(|db, body: CreateUserRequest| create_user(db, body.email, body.password))
        .recover(handle_errors);

    let users_routes = warp::path("users").and(get_users_route.or(create_user_route));

    status.or(users_routes)
}
