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
pub async fn posts(sort: &str, search: &str, offset: i64) -> Result<Vec<Post>, reqwest::Error> {
    reqwest::get(format!(
        "{API}/posts?limit=12&offset={offset}&sort={sort}&search={search}"
    ))
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
pub async fn create_post(token: &str, body: PostInput) -> Result<Post, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("{API}/posts"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn update_post(id: i64, token: &str, body: PostInput) -> Result<Post, reqwest::Error> {
    reqwest::Client::new()
        .put(format!("{API}/posts/{id}"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn delete_post(id: i64, token: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(format!("{API}/posts/{id}"))
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
pub async fn update_comment(
    post_id: i64,
    comment_id: i64,
    token: &str,
    content: String,
) -> Result<Comment, reqwest::Error> {
    reqwest::Client::new()
        .put(format!("{API}/posts/{post_id}/comments/{comment_id}"))
        .bearer_auth(token)
        .json(&CreateCommentRequest { content })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn delete_comment(
    post_id: i64,
    comment_id: i64,
    token: &str,
) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(format!("{API}/posts/{post_id}/comments/{comment_id}"))
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
pub async fn me(token: &str) -> Result<User, reqwest::Error> {
    reqwest::Client::new()
        .get(format!("{API}/users/me"))
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
pub async fn change_password(
    token: &str,
    body: PasswordChangeRequest,
) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .put(format!("{API}/users/me/password"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
