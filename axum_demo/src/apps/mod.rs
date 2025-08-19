use axum::{routing::get, Json, Router};
use crate::apps::note::router::router as note_router;

pub mod note;

#[derive(serde::Serialize)]
struct User {
    id: i32,
}

async fn get_user() -> Json<User> {
    Json(User { id: 1 })
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello from Apps!" }))
        .route("/user", get(get_user))
        .nest("/note", note_router())
}