use std::fs;
use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
    http::header,
};
use tokio::net::TcpListener;

async fn serve_gif() -> Response {
    let data = fs::read(r"C:\Users\Valeriia Morkhat\IdeaProjects\meetup\src\files\sticker.gif").expect("Failed to read sticker.gif");
    (
        [(header::CONTENT_TYPE, "image/gif")],
        data
    ).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(serve_gif));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}