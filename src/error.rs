use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Authentication error")]
    Auth(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Auth(_) => StatusCode::UNAUTHORIZED,
        };
        (status, self.to_string()).into_response()
    }
}
