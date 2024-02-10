use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serde json error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
