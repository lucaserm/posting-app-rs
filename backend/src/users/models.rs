use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(sqlx::FromRow)]
pub struct UserCredentials {
    pub password_hash: String,
}
