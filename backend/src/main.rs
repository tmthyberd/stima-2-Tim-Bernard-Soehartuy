mod algorithms;
mod models;
mod parser;
mod routes;
mod scraper;
mod selectors;
use crate::routes::{handle_parse, handle_scrape, handle_search};
use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/parse", post(handle_parse))
        .route("/api/scrape", post(handle_scrape))
        .route("/api/search", post(handle_search))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server berjalan di http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    r#"{"status": "ok"}"#
}
