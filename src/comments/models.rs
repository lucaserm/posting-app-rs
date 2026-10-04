use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct ListCommentsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
