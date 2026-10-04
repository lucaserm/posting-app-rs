use axum::{Router, routing::post};

use crate::{
    auth::handlers::{login, register},
    state::AppState,
};

mod extractor;
mod models;
mod password;
mod token;

pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
