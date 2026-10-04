mod auth;
mod comments;
mod error;
mod health;
mod posts;
mod state;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to postgresql");

    sqlx::migrate!("./src/migrations")
        .run(&pool)
        .await
        .expect("failed to run database migrations");

    println!("connected to postgresql");

    let state = AppState::new(pool, jwt_secret);

    let app = Router::new()
        .route("/health", get(health::health))
        .nest("/posts", posts::routes())
        .nest("/auth", auth::routes())
        .route("/users/me", get(auth::handlers::me))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind port 3000");

    println!("server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server failed");
}
