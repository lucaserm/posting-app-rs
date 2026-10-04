mod handlers;
mod models;
mod validation;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    comments::handlers::{create_comment, delete_comment, list_comments, update_comment},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{post_id}/comments", get(list_comments))
        .route("/{post_id}/comments", post(create_comment))
        .route("/{post_id}/comments/{comment_id}", put(update_comment))
        .route("/{post_id}/comments/{comment_id}", delete(delete_comment))
}
