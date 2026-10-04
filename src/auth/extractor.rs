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

#[cfg(test)]
mod tests {
    use axum::{
        extract::FromRequestParts,
        http::{Request, header::AUTHORIZATION},
    };
    use sqlx::postgres::PgPoolOptions;

    use super::AuthUser;
    use crate::{auth::token::issue_access_token, state::AppState};

    fn test_state() -> AppState {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://posts:posts@localhost/posts_api")
            .unwrap();

        AppState {
            pool,
            jwt_secret: String::from("unit-test-secret"),
        }
    }

    #[tokio::test]
    async fn accepts_a_valid_bearer_token() {
        let state = test_state();
        let token = issue_access_token(42, &state.jwt_secret).unwrap();

        let request = Request::builder()
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .body(())
            .unwrap();
        let (mut parts, _) = request.into_parts();

        let result = AuthUser::from_request_parts(&mut parts, &state).await;

        assert!(matches!(result, Ok(AuthUser { user_id: 42 })));
    }

    #[tokio::test]
    async fn rejects_a_missing_token() {
        let state = test_state();
        let request = Request::new(());
        let (mut parts, _) = request.into_parts();

        let result = AuthUser::from_request_parts(&mut parts, &state).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn rejects_an_invalid_token() {
        let state = test_state();
        let request = Request::builder()
            .header(AUTHORIZATION, "Bearer invalid-token")
            .body(())
            .unwrap();
        let (mut parts, _) = request.into_parts();

        let result = AuthUser::from_request_parts(&mut parts, &state).await;

        assert!(result.is_err());
    }
}
