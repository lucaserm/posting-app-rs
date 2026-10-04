use crate::types::*;
const API: &str = "http://127.0.0.1:3000";
pub async fn login(body: &LoginRequest) -> Result<LoginResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("{API}/auth/login"))
        .json(body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn posts(sort: &str) -> Result<Vec<Post>, reqwest::Error> {
    reqwest::get(format!("{API}/posts?limit=30&offset=0&sort={sort}"))
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn post(id: i64) -> Result<Post, reqwest::Error> {
    reqwest::get(format!("{API}/posts/{id}"))
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn comments(id: i64) -> Result<Vec<Comment>, reqwest::Error> {
    reqwest::get(format!("{API}/posts/{id}/comments?limit=50&offset=0"))
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn create_comment(
    id: i64,
    token: &str,
    content: String,
) -> Result<Comment, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("{API}/posts/{id}/comments"))
        .bearer_auth(token)
        .json(&CreateCommentRequest { content })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
