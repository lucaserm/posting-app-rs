use axum::{Json, extract::State, http::StatusCode};

use crate::{error::ApiError, state::AppState};

use super::extractor::AuthUser;
use super::models::{LoginRequest, LoginResponse, UserCredentials};
use super::password::verify_password;
use super::{
    models::{RegisterRequest, UserResponse},
    password::hash_password,
};

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let email = input.email.trim().to_lowercase();

    if !email.contains('@') {
        return Err(ApiError::InvalidInput(String::from("email is invalid")));
    }

    if input.password.len() < 12 {
        return Err(ApiError::InvalidInput(String::from(
            "password must be at least 12 bytes",
        )));
    }

    let password_hash = hash_password(&input.password).map_err(|error| {
        eprintln!("password hashing error: {error}");
        ApiError::InternalError
    })?;

    let user = sqlx::query_as::<_, UserResponse>(
        "INSERT INTO users (email, password_hash)
         VALUES ($1, $2)
         RETURNING id, email",
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let email = input.email.trim().to_lowercase();

    let user = sqlx::query_as::<_, UserCredentials>(
        "SELECT id, password_hash FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::Unauthorized(Some(String::from(
        "invalid credentials",
    ))))?;

    if verify_password(&input.password, &user.password_hash).is_err() {
        return Err(ApiError::Unauthorized(Some(String::from(
            "invalid credentials",
        ))));
    }

    let expires_in = 60 * 60; // expires in 1h

    let access_token =
        super::token::issue_access_token(user.id, &state.jwt_secret).map_err(|error| {
            eprintln!("JWT signing error: {error}");
            ApiError::InternalError
        })?;

    Ok(Json(LoginResponse {
        access_token,
        token_type: String::from("Bearer"),
        expires_in,
    }))
}

pub async fn me(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<UserResponse>, ApiError> {
    let user = sqlx::query_as::<_, UserResponse>("SELECT id, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(ApiError::Unauthorized(None))?;

    Ok(Json(user))
}
