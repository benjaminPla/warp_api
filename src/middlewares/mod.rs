use sqlx::{Pool, Postgres};
use warp::Filter;

pub fn with_db(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// pub fn authenticate(
// pool: Pool<Postgres>,
// headers : warp::header::<String>("Authorization")
// ) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
// warp::any().map(|auth_header: String| {
// let is_valid =
// })
// }
