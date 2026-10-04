use axum::{Router, routing::put};

use crate::{state::AppState, users::handlers::change_password};

mod handlers;
mod models;

pub fn routes() -> Router<AppState> {
    Router::new().route("/me/password", put(change_password))
}
