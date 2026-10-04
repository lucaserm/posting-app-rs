use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{auth::extractor::AuthUser, error::ApiError, state::AppState};

use super::{
    models::{Comment, CreateCommentRequest, ListCommentsQuery, UpdateCommentRequest},
    validation::validate_comment,
};

pub async fn list_comments(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
    Query(query): Query<ListCommentsQuery>,
) -> Result<Json<Vec<Comment>>, ApiError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    if !(1..=100).contains(&limit) {
        return Err(ApiError::InvalidInput(String::from(
            "limit must be between 1 and 100",
        )));
    }

    if offset < 0 {
        return Err(ApiError::InvalidInput(String::from(
            "offset cannot be negative",
        )));
    }

    let post_exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
            .bind(post_id)
            .fetch_one(&state.pool)
            .await?;

    if !post_exists {
        return Err(ApiError::NotFound);
    }

    let comments = sqlx::query_as::<_, Comment>(
        "SELECT id, user_id, content, created_at
         FROM comments
         WHERE post_id = $1
         ORDER BY created_at DESC, id DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(post_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(comments))
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
    AuthUser { user_id }: AuthUser,
    Json(input): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), ApiError> {
    let content = input.content.trim();
    validate_comment(content)?;

    let post_exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
            .bind(post_id)
            .fetch_one(&state.pool)
            .await?;

    if !post_exists {
        return Err(ApiError::NotFound);
    }

    let comment = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (post_id, user_id, content)
         VALUES ($1, $2, $3)
         RETURNING id, user_id, content, created_at",
    )
    .bind(post_id)
    .bind(user_id)
    .bind(content)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn update_comment(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
    Path(comment_id): Path<i64>,
    AuthUser { user_id }: AuthUser,
    Json(input): Json<UpdateCommentRequest>,
) -> Result<Json<Comment>, ApiError> {
    let content = input.content.trim();
    validate_comment(content)?;

    let post_exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
            .bind(post_id)
            .fetch_one(&state.pool)
            .await?;

    if !post_exists {
        return Err(ApiError::NotFound);
    }

    let comment = sqlx::query_as::<_, Comment>(
        "UPDATE comments
        SET content = $1
        WHERE id = $2
        AND post_id = $3
        AND user_id = $4
        RETURNING id, user_id, content, created_at",
    )
    .bind(content)
    .bind(comment_id)
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(comment))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
    Path(comment_id): Path<i64>,
    AuthUser { user_id }: AuthUser,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query(
        "DELETE FROM comments
        WHERE id = $1
        AND post_id = $2
        AND (user_id = $3
        OR user_id = (SELECT author_id FROM posts WHERE id = $2))",
    )
    .bind(comment_id)
    .bind(post_id)
    .bind(user_id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
