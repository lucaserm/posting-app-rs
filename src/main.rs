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

    let app = Router::new()
        .route("/health", get(health::health))
        .nest("/posts", posts::routes())
        .with_state(AppState::new(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind port 3000");

    println!("server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server failed");
}
