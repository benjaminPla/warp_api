use sqlx::{Pool, Postgres};
use warp::Filter;

pub fn with_db(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
