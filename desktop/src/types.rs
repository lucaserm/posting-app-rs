use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, PartialEq)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub comment_count: i64,
}
#[derive(Clone, Deserialize, PartialEq)]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
}
#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}
#[derive(Serialize)]
pub struct CreateCommentRequest {
    pub content: String,
}
#[derive(Serialize)]
pub struct PostInput {
    pub title: String,
    pub content: String,
}
#[derive(Serialize)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
}
#[derive(Clone, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub email: String,
}
