// use serde::{Deserialize, Serialize};
// use utoipa::{ToSchema, OpenApi};
// use utoipa_swagger_ui::SwaggerUi;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize, ToSchema)]
// struct Note {
//     id: Uuid,
//     title: String,
//     content: String,
// }

// #[utoipa::path(
//     get,
//     path = "/notes/{id}",
//     responses(
//         (status = 200, description = "Get a note", body = Note)
//     )
// )]
// async fn get_note() -> Json<Note> {
//     Json(Note {
//         id: Uuid::new_v4(),
//         title: "Hello".into(),
//         content: "Rust + Axum + utoipa!".into(),
//     })
// }

// #[derive(OpenApi)]
// #[openapi(paths(get_note), components(schemas(Note)))]
// struct ApiDoc;

mod apps; // 告诉 Rust 编译器有 apps 模块

use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_axum::{routes, PathItemExt, router::UtoipaMethodRouter, router::OpenApiRouter};
use utoipa_scalar::{Scalar, Servable};

#[derive(utoipa::ToSchema, serde::Serialize)]
struct User {
    id: i32,
}

#[utoipa::path(
    get, 
    path = "/user", 
    responses(
        (status = 200, description = "Get user", body = User)
    )
)]
async fn get_user() -> Json<User> {
    Json(User { id: 1 })
}

#[derive(OpenApi)]
#[openapi(paths(get_user), components(schemas(User)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let openapi_router: OpenApiRouter<()> = OpenApiRouter::new()
    .route("/user", get(get_user));
    let (router, openapi) = openapi_router.split_for_parts();
    let app = router
        .route("/openapi.json", get({
            Json(openapi.clone())
        }))
        .merge(Scalar::with_url("/scalar", openapi));
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("🚀 Server running at http://127.0.0.1:3000");
    println!("📖 API routes: GET /user");
    axum::serve(listener, app).await.unwrap();
}




