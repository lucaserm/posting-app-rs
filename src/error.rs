use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, String::from("resource not found")),
            ApiError::InvalidInput(message) => (StatusCode::BAD_REQUEST, message),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("internal server error"),
            ),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        eprintln!("Database error: {error}");
        Self::InternalError
    }
}
