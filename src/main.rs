use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::net::TcpListener;
use tracing_subscriber;
use tracing;

async fn serve_gif() -> Response {
    let data = fs::read(r"C:\Users\Valeriia Morkhat\IdeaProjects\meetup\src\files\sticker.gif").expect("Failed to read sticker.gif");
    (
        [(header::CONTENT_TYPE, "image/gif")],
        data
    ).into_response()
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user)).into_response()
}

async fn get_users() -> Response {
        let users = vec![
            User { id: 1, username: "user1".to_string() },
            User { id: 2, username: "user2".to_string() },
        ];

        Json(users).into_response()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(serve_gif))
        .route("/users", get(get_users))
        .route("/users", post(create_user));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}