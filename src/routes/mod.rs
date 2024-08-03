use crate::middlewares::with_db;
use sqlx::{Pool, Postgres};
use warp::Filter;
use crate::controllers::get_users;

pub fn create_routes(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let status = warp::path::end().map(|| "Up");


    let get_users = warp::path("get_users")
        .and(with_db(pool.clone()))
        .and_then(get_users);

    warp::get().and(status.or(get_users))
}
