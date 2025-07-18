use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from repox (access tokens endpoint)!".into(),
    })
}
