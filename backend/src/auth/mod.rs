use axum::{Router, routing::post};

use crate::{
    auth::handlers::{login, register},
    state::AppState,
};

pub mod extractor;
pub mod handlers;
pub mod token;

mod models;
mod password;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
