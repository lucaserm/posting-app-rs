use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    error::ApiError,
    posts::{
        models::{ListPostsQuery, UpdatePostRequest},
        validation::validate_post,
    },
    state::AppState,
};

use super::models::{CreatePostRequest, Post};

pub async fn create_post(
    State(state): State<AppState>,
    Json(input): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), ApiError> {
    let title = input.title.trim();
    let content = input.content.trim();

    validate_post(title, content)?;

    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content)
        VALUES ($1, $2)
        RETURNING id, title, content",
    )
    .bind(title)
    .bind(content)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn list_posts(
    State(state): State<AppState>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<Vec<Post>>, ApiError> {
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

    let posts = sqlx::query_as::<_, Post>(
        "SELECT id, title, content
        FROM posts
        ORDER BY id DESC
        LIMIT $1
        OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(posts))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Post>, ApiError> {
    let post = sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(post))
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdatePostRequest>,
) -> Result<Json<Post>, ApiError> {
    let title = input.title.trim();
    let content = input.content.trim();

    validate_post(title, content)?;

    let post = sqlx::query_as::<_, Post>(
        "UPDATE posts
        SET title = $1, content = $2
        WHERE id = $3
        RETURNING id, title, content",
    )
    .bind(title)
    .bind(content)
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(post))
}
