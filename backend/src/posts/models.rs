use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListPostsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort: Option<String>,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub comment_count: i64,
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}
