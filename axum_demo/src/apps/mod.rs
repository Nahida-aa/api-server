use axum::{routing::get, Router};

use crate::apps::note::router::router as note_router;

pub mod note;

pub fn app() -> Router {
    Router::new()
        .route("/", get( || async { 
            return  "Hello, Axum!";
        } ))
        .nest("/note", note_router())
}