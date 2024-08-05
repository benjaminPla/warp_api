use crate::controllers::{authenticate, create_user, delete_user, get_users, update_user};
use crate::errors::handle_errors;
use crate::middlewares::with_db;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use warp::Filter;

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
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
        .and(warp::body::json::<UserRequest>())
        .and_then(create_user)
        .recover(handle_errors);

    let update_user_route = warp::path("update_user")
        .and(warp::put())
        .and(with_db(pool.clone()))
        .and(warp::path::param())
        .and(warp::body::json::<UserRequest>())
        .and_then(update_user)
        .recover(handle_errors);

    let delete_user_route = warp::path("delete_user")
        .and(warp::delete())
        .and(with_db(pool.clone()))
        .and(warp::path::param())
        .and_then(delete_user)
        .recover(handle_errors);

    let users_routes = warp::path("users").and(
        get_users_route
            .or(create_user_route)
            .or(delete_user_route)
            .or(update_user_route),
    );

    let authenticate_route = warp::path("authenticate")
        .and(warp::post())
        .and(with_db(pool.clone()))
        // .and(warp::header::<String>("Authorization"))
        .and(warp::body::json::<UserRequest>())
        .and_then(authenticate)
        .recover(handle_errors);

    status.or(authenticate_route).or(users_routes)
}
