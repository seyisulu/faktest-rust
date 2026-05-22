use axum::{Router, routing::get, response::Html};
use tower_http::trace::TraceLayer;
use tracing_subscriber;

mod config;
mod db;
mod error;
mod models;
mod handlers;
mod services;
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { Html("<h1>FakTest Rust Backend</h1>") }))
        .nest("/api", handlers::routes())
        .layer(TraceLayer::new_for_http());

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("FakTest backend running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
