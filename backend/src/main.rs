// Memberitahu Rust bahwa kita punya modul-modul ini
mod models;
mod parser;
mod scraper;
mod algorithms;

// "use" = import, seperti "import" di TypeScript
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

// Fungsi utama — #[tokio::main] artinya:
// "jalankan fungsi ini sebagai async program menggunakan tokio runtime"
#[tokio::main]
async fn main() {
    // Membuat "router" — peta URL ke fungsi handler
    let app = Router::new()
        .route("/health", get(health_check)) // GET /health → panggil fungsi health_check
        .layer(CorsLayer::permissive());     // Izinkan request dari frontend (CORS)

    // Tentukan alamat server: localhost port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap(); // .unwrap() = "kalau error, crash program" (ok untuk sekarang)

    println!("🚀 Server berjalan di http://localhost:8080");

    // Jalankan server, terus sampai di-stop
    axum::serve(listener, app).await.unwrap();
}

// Handler untuk route GET /health
// "async fn" karena handler di axum harus async
async fn health_check() -> &'static str {
    // Mengembalikan string JSON sederhana
    r#"{"status": "ok"}"#
}