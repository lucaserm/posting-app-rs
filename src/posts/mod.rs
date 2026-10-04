mod handlers;
pub mod models;
mod validation;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{posts::handlers::update_post, state::AppState};
use handlers::{create_post, delete_post, get_post, list_posts};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/", get(list_posts))
        .route("/{id}", get(get_post))
        .route("/{id}", delete(delete_post))
        .route("/{id}", put(update_post))
}
