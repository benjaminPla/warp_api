use crate::errors::{ServerError, UnauthorizedTypes};
use crate::helpers::{verify_token, TokenValidationError};
use sqlx::{Pool, Postgres};
use warp::{Filter, Rejection};

pub fn db_middleware(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn authenticate_middleware() -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| async move {
        match verify_token(token) {
            Ok(_) => Ok(()),
            Err(TokenValidationError::Expired) => Err(warp::reject::custom(
                ServerError::Unauthorized(UnauthorizedTypes::TokenExpired),
            )),
            Err(TokenValidationError::Invalid) => Err(warp::reject::custom(
                ServerError::Unauthorized(UnauthorizedTypes::TokenInvalid),
            )),
            Err(TokenValidationError::Other) => {
                Err(warp::reject::custom(ServerError::InternalServerError))
            }
        }
    })
}
