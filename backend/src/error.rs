use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum ApiError {
    NotFound,
    InvalidInput(String),
    Unauthorized(Option<String>),
    SessionExpired,
    Conflict(String),
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
            ApiError::Unauthorized(message) => (
                StatusCode::UNAUTHORIZED,
                match message {
                    Some(message) => message,
                    None => String::from("unauthorized"),
                },
            ),
            ApiError::SessionExpired => (
                StatusCode::from_u16(498).expect("498 is a valid HTTP status code"),
                String::from("session expired"),
            ),
            ApiError::Conflict(message) => (StatusCode::CONFLICT, message),
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
        if let sqlx::Error::Database(database_error) = &error
            && database_error.constraint() == Some("users_email_key")
        {
            return Self::Conflict(String::from("email is already registered"));
        }

        eprintln!("Database error: {error}");
        Self::InternalError
    }
}
