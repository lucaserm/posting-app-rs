use axum::{Json, extract::State, http::StatusCode};

use crate::{
    auth::{
        extractor::AuthUser,
        password::{hash_password, verify_password},
    },
    error::ApiError,
    state::AppState,
    users::models::UserCredentials,
};

use super::models::UpdatePasswordRequest;

pub async fn change_password(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(input): Json<UpdatePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    if input.new_password.len() < 12 {
        return Err(ApiError::InvalidInput(String::from(
            "password must be at least 12 bytes",
        )));
    }

    let user =
        sqlx::query_as::<_, UserCredentials>("SELECT password_hash FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&state.pool)
            .await?;

    if verify_password(&input.current_password, &user.password_hash).is_err() {
        return Err(ApiError::Unauthorized(Some(String::from(
            "current password does not match",
        ))));
    }

    let password_hash = hash_password(&input.new_password).map_err(|error| {
        eprintln!("password hashing error: {error}");
        ApiError::InternalError
    })?;

    let result = sqlx::query(
        "UPDATE users
        SET password_hash = $1
        WHERE id = $2",
    )
    .bind(password_hash)
    .bind(user_id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::InternalError);
    }

    Ok(StatusCode::NO_CONTENT)
}
