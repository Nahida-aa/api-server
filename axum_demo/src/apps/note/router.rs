use axum::{routing::{get, post}, Router};

pub fn router() -> Router {
    Router::new()
    .route("/", get(|| async { "get note" }))
    .route("/", post(|| async { "create note" }))
}