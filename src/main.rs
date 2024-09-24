use axum::{Router, serve};
use tokio::net::TcpListener;

mod router;
mod renderer;

#[tokio::main]
async fn main() {
    let engine = renderer::create();

    let app = Router::from(router::create(engine));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
