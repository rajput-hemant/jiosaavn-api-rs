pub mod handlers;
pub mod models;
pub mod payloads;
pub mod services;
pub mod utils;

use axum::{http::Method, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use handlers::{album_handler, modules_handler, playlist_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        // home modules / launch data
        .route("/modules", get(modules_handler::modules_handler))
        // album details route
        .route("/album", get(album_handler::album_details_handler))
        .route(
            "/album/recommendations",
            get(album_handler::recommend_albums_handler),
        )
        // playlist details route
        .route("/playlist", get(playlist_handler::playlist_details_handler))
        // cors layer
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("🚀 Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "🚀 Welcome to Unofficial Jio Saavn API!"
}
