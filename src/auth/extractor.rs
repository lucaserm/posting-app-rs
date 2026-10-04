use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

use crate::{error::ApiError, state::AppState};

use super::token::verify_access_token;

#[derive(Clone, Copy)]
pub struct AuthUser {
    pub user_id: i64,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let value = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(ApiError::Unauthorized(None))?;

        let (scheme, token) = value.split_once(' ').ok_or(ApiError::Unauthorized(None))?;

        if !scheme.eq_ignore_ascii_case("bearer") || token.is_empty() {
            return Err(ApiError::Unauthorized(None));
        }

        let claims = verify_access_token(token, &state.jwt_secret)
            .map_err(|_| ApiError::Unauthorized(None))?;

        let user_id = claims
            .sub
            .parse::<i64>()
            .map_err(|_| ApiError::Unauthorized(None))?;

        Ok(Self { user_id })
    }
}
