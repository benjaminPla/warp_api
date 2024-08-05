use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug)]
pub struct InternalServerError;
impl warp::reject::Reject for InternalServerError {}

pub async fn handle_errors(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_) = err.find::<InternalServerError>() {
        return Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    Err(err)
}
