use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    message: String,
    version: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: String::from("ok"),
        message: String::from("api is running"),
        version: String::from("v0.1.0"),
    })
}
