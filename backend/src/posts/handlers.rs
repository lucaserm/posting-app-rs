use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    auth::extractor::AuthUser,
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
    AuthUser { user_id }: AuthUser,
    Json(input): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), ApiError> {
    let title = input.title.trim();
    let content = input.content.trim();

    validate_post(title, content)?;

    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content, author_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, 0::BIGINT AS comment_count",
    )
    .bind(title)
    .bind(content)
    .bind(user_id)
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
    let sort = query.sort.as_deref().unwrap_or("recent");
    let search = query
        .search
        .as_deref()
        .map(str::trim)
        .filter(|term| !term.is_empty());

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

    if !matches!(sort, "recent" | "discussed") {
        return Err(ApiError::InvalidInput(String::from(
            "sort must be either recent or discussed",
        )));
    }

    let order_by = if sort == "discussed" {
        "comment_count DESC, posts.id DESC"
    } else {
        "posts.id DESC"
    };

    let query = format!(
        "SELECT posts.id, posts.title, posts.content, COUNT(comments.id)::BIGINT AS comment_count
         FROM posts
         LEFT JOIN comments ON comments.post_id = posts.id
         WHERE (
            $1::TEXT IS NULL
            OR posts.title ILIKE '%' || $1 || '%'
            OR posts.content ILIKE '%' || $1 || '%'
         )
         GROUP BY posts.id
         ORDER BY {order_by}
         LIMIT $2 OFFSET $3"
    );

    let posts = sqlx::query_as::<_, Post>(&query)
        .bind(search)
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
    let post = sqlx::query_as::<_, Post>(
        "SELECT posts.id, posts.title, posts.content, COUNT(comments.id)::BIGINT AS comment_count
         FROM posts LEFT JOIN comments ON comments.post_id = posts.id
         WHERE posts.id = $1 GROUP BY posts.id",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(post))
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    AuthUser { user_id }: AuthUser,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1 AND author_id = $2")
        .bind(id)
        .bind(user_id)
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
    AuthUser { user_id }: AuthUser,
    Json(input): Json<UpdatePostRequest>,
) -> Result<Json<Post>, ApiError> {
    let title = input.title.trim();
    let content = input.content.trim();

    validate_post(title, content)?;

    let post = sqlx::query_as::<_, Post>(
        "UPDATE posts
        SET title = $1, content = $2
        WHERE id = $3 AND author_id = $4
        RETURNING id, title, content, 0::BIGINT AS comment_count",
    )
    .bind(title)
    .bind(content)
    .bind(id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(post))
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::{
            Method, Request, StatusCode,
            header::{AUTHORIZATION, CONTENT_TYPE},
        },
        response::Response,
    };
    use serde_json::Value;
    use sqlx::{PgPool, Row};
    use tower::ServiceExt;

    use crate::{auth::token::issue_access_token, posts::routes, state::AppState};

    const TEST_SECRET: &str = "test-secret-for-post-handler-tests";

    fn test_app(pool: PgPool) -> Router {
        Router::new().nest("/posts", routes()).with_state(AppState {
            pool,
            jwt_secret: TEST_SECRET.to_owned(),
        })
    }

    async fn create_test_user(pool: &PgPool, email: &str) -> i64 {
        sqlx::query(
            "INSERT INTO users (email, password_hash)
             VALUES ($1, 'test-hash')
             RETURNING id",
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .unwrap()
        .get("id")
    }

    fn make_request(method: Method, uri: &str, token: Option<&str>, body: &str) -> Request<Body> {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json");

        if let Some(token) = token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
        }

        builder.body(Body::from(body.to_owned())).unwrap()
    }

    async fn send(
        app: &Router,
        method: Method,
        uri: &str,
        token: Option<&str>,
        body: &str,
    ) -> Response {
        app.clone()
            .oneshot(make_request(method, uri, token, body))
            .await
            .unwrap()
    }

    #[sqlx::test(migrations = "./src/migrations")]
    async fn post_routes_enforce_authentication_and_ownership(pool: PgPool) {
        let owner_id = create_test_user(&pool, "owner@example.test").await;
        let other_id = create_test_user(&pool, "other@example.test").await;

        let owner_token = issue_access_token(owner_id, TEST_SECRET).unwrap();
        let other_token = issue_access_token(other_id, TEST_SECRET).unwrap();
        let app = test_app(pool);

        let unauthenticated = send(
            &app,
            Method::POST,
            "/posts",
            None,
            r#"{"title":"Test post","content":"Test content"}"#,
        )
        .await;
        assert_eq!(unauthenticated.status(), StatusCode::UNAUTHORIZED);

        let created = send(
            &app,
            Method::POST,
            "/posts",
            Some(&owner_token),
            r#"{"title":"Test post","content":"Test content"}"#,
        )
        .await;
        assert_eq!(created.status(), StatusCode::CREATED);

        let body = to_bytes(created.into_body(), usize::MAX).await.unwrap();
        let post: Value = serde_json::from_slice(&body).unwrap();
        let post_id = post["id"].as_i64().unwrap();

        let other_update = send(
            &app,
            Method::PUT,
            &format!("/posts/{post_id}"),
            Some(&other_token),
            r#"{"title":"Changed","content":"Changed"}"#,
        )
        .await;
        assert_eq!(other_update.status(), StatusCode::NOT_FOUND);

        let other_delete = send(
            &app,
            Method::DELETE,
            &format!("/posts/{post_id}"),
            Some(&other_token),
            "",
        )
        .await;
        assert_eq!(other_delete.status(), StatusCode::NOT_FOUND);

        let owner_update = send(
            &app,
            Method::PUT,
            &format!("/posts/{post_id}"),
            Some(&owner_token),
            r#"{"title":"Updated","content":"Updated content"}"#,
        )
        .await;
        assert_eq!(owner_update.status(), StatusCode::OK);

        let owner_delete = send(
            &app,
            Method::DELETE,
            &format!("/posts/{post_id}"),
            Some(&owner_token),
            "",
        )
        .await;
        assert_eq!(owner_delete.status(), StatusCode::NO_CONTENT);
    }
}
