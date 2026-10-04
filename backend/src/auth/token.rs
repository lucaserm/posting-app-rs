use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

pub fn issue_access_token(
    user_id: i64,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_secs();

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + 60 * 60,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_access_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::{issue_access_token, verify_access_token};

    #[test]
    fn issued_token_contains_user_id_and_expiry() {
        let token = issue_access_token(42, "test-secret").unwrap();
        let claims = verify_access_token(&token, "test-secret").unwrap();

        assert_eq!(claims.sub, "42");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn rejects_token_signed_with_a_different_secret() {
        let token = issue_access_token(42, "test-secret").unwrap();

        assert!(verify_access_token(&token, "wrong-secret").is_err());
    }
}
