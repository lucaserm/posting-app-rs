use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};

use crate::types::*;
const API: &str = "http://127.0.0.1:3000";

#[derive(Deserialize)]
struct ApiErrorBody {
    error: String,
}

pub enum ApiClientError {
    Validation(String),
    Unauthorized(String),
    SessionExpired(String),
    NotFound(String),
    Forbidden(String),
    Server(String),
    Network(String),
}

impl ApiClientError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Validation(message) => message.clone(),
            Self::Unauthorized(message) => format!("Authentication is required: {message}"),
            Self::SessionExpired(message) => {
                format!("Your session has expired. Please sign in again: {message}")
            }
            Self::Forbidden(message) => {
                format!("You do not have permission for this action: {message}")
            }
            Self::NotFound(message) => format!("The requested resource was not found: {message}"),
            Self::Server(message) => {
                format!("The server could not complete the request: {message}")
            }
            Self::Network(message) => format!("Could not reach the API: {message}"),
        }
    }
}

async fn read_json<T: DeserializeOwned>(response: Response) -> Result<T, ApiClientError> {
    let status = response.status();

    if status.is_success() {
        return response
            .json::<T>()
            .await
            .map_err(|error| ApiClientError::Network(error.to_string()));
    }

    let message = response
        .json::<ApiErrorBody>()
        .await
        .map(|body| body.error)
        .unwrap_or_else(|_| "Unexpected API error".to_string());

    match status.as_u16() {
        498 => Err(ApiClientError::SessionExpired(message)),
        _ => match status {
            StatusCode::BAD_REQUEST => Err(ApiClientError::Validation(message)),
            StatusCode::UNAUTHORIZED => Err(ApiClientError::Unauthorized(message)),
            StatusCode::FORBIDDEN => Err(ApiClientError::Forbidden(message)),
            StatusCode::NOT_FOUND => Err(ApiClientError::NotFound(message)),
            _ => Err(ApiClientError::Server(message)),
        },
    }
}

async fn read_empty(response: Response) -> Result<(), ApiClientError> {
    let status = response.status();

    if status.is_success() {
        return Ok(());
    }

    let message = response
        .json::<ApiErrorBody>()
        .await
        .map(|body| body.error)
        .unwrap_or_else(|_| "Unexpected API error".to_string());

    match status.as_u16() {
        498 => Err(ApiClientError::SessionExpired(message)),
        _ => match status {
            StatusCode::BAD_REQUEST => Err(ApiClientError::Validation(message)),
            StatusCode::UNAUTHORIZED => Err(ApiClientError::Unauthorized(message)),
            StatusCode::FORBIDDEN => Err(ApiClientError::Forbidden(message)),
            StatusCode::NOT_FOUND => Err(ApiClientError::NotFound(message)),
            _ => Err(ApiClientError::Server(message)),
        },
    }
}

pub async fn login(body: &LoginRequest) -> Result<LoginResponse, ApiClientError> {
    let response = reqwest::Client::new()
        .post(format!("{API}/auth/login"))
        .json(body)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn register(body: &LoginRequest) -> Result<User, ApiClientError> {
    let response = reqwest::Client::new()
        .post(format!("{API}/auth/register"))
        .json(body)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn posts(sort: &str, search: &str, offset: i64) -> Result<Vec<Post>, ApiClientError> {
    let response = reqwest::Client::new()
        .get(format!(
            "{API}/posts?limit=12&offset={offset}&sort={sort}&search={search}"
        ))
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn post(id: i64) -> Result<Post, ApiClientError> {
    let response = reqwest::get(format!("{API}/posts/{id}"))
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn comments(id: i64) -> Result<Vec<Comment>, ApiClientError> {
    let response = reqwest::get(format!("{API}/posts/{id}/comments?limit=50&offset=0"))
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn create_comment(
    id: i64,
    token: &str,
    content: String,
) -> Result<Comment, ApiClientError> {
    let response = reqwest::Client::new()
        .post(format!("{API}/posts/{id}/comments"))
        .bearer_auth(token)
        .json(&CreateCommentRequest { content })
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn create_post(token: &str, body: PostInput) -> Result<Post, ApiClientError> {
    let response = reqwest::Client::new()
        .post(format!("{API}/posts"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn update_post(id: i64, token: &str, body: PostInput) -> Result<Post, ApiClientError> {
    let response = reqwest::Client::new()
        .put(format!("{API}/posts/{id}"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn delete_post(id: i64, token: &str) -> Result<(), ApiClientError> {
    let response = reqwest::Client::new()
        .delete(format!("{API}/posts/{id}"))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_empty(response).await
}
pub async fn update_comment(
    post_id: i64,
    comment_id: i64,
    token: &str,
    content: String,
) -> Result<Comment, ApiClientError> {
    let response = reqwest::Client::new()
        .put(format!("{API}/posts/{post_id}/comments/{comment_id}"))
        .bearer_auth(token)
        .json(&CreateCommentRequest { content })
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn delete_comment(
    post_id: i64,
    comment_id: i64,
    token: &str,
) -> Result<(), ApiClientError> {
    let response = reqwest::Client::new()
        .delete(format!("{API}/posts/{post_id}/comments/{comment_id}"))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_empty(response).await
}
pub async fn me(token: &str) -> Result<User, ApiClientError> {
    let response = reqwest::Client::new()
        .get(format!("{API}/users/me"))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|error| ApiClientError::Network(error.to_string()))?;
    read_json(response).await
}
pub async fn change_password(
    token: &str,
    body: PasswordChangeRequest,
) -> Result<(), ApiClientError> {
    let response = reqwest::Client::new()
        .put(format!("{API}/users/me/password"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .map_err(|err| ApiClientError::Network(err.to_string()))?;

    read_empty(response).await
}
