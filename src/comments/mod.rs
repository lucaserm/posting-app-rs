mod handlers;
mod models;
mod validation;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    comments::handlers::{create_comment, list_comments},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{post_id}/comments", get(list_comments))
        .route("/{post_id}/comments", post(create_comment))
}
