
mod models;
mod parser;
mod scraper;
mod algorithms;


use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/health", get(health_check)) 
        .layer(CorsLayer::permissive());     

    // Tentukan alamat server: localhost port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server berjalan di http://localhost:8080");


    axum::serve(listener, app).await.unwrap();
}


async fn health_check() -> &'static str {
    r#"{"status": "ok"}"#
}