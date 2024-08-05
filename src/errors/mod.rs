use warp::{http::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Debug)]
pub enum ServerError {
    InternalServerError,
    NotFound,
}

impl ServerError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServerError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ServerError::InternalServerError => "Internal Server Error",
            ServerError::NotFound => "Not Found",
        }
    }
}

impl Reject for ServerError {}

pub async fn handle_errors(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(server_error) = err.find::<ServerError>() {
        return Ok(warp::reply::with_status(
            server_error.message().to_string(),
            server_error.status_code(),
        ));
    }

    Err(err)
}
