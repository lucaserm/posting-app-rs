use axum::{Router, routing::post};

use crate::{
    auth::handlers::{login, register},
    state::AppState,
};

pub mod extractor;
pub mod handlers;
pub mod password;
pub mod token;

mod models;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
