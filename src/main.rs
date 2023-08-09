pub mod handlers;
pub mod models;
pub mod payloads;
pub mod services;
pub mod utils;

use axum::{http::Method, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use handlers::{
    album_details_handler, artist_albums_handler, artist_details_handler, artist_songs_handler,
    modules_handler, playlist_details_handler, recommend_albums_handler,
    recommend_artists_songs_handler, recommend_songs_handler, song_details_handler,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        // home modules / launch data
        .route("/modules", get(modules_handler))
        // song details route
        .route("/song", get(song_details_handler))
        .route("/song/recommendations", get(recommend_songs_handler))
        // album details route
        .route("/album", get(album_details_handler))
        .route("/album/recommendations", get(recommend_albums_handler))
        // playlist details route
        .route("/playlist", get(playlist_details_handler))
        // artist details route
        .route("/artist", get(artist_details_handler))
        .route("/artist/songs", get(artist_songs_handler))
        .route("/artist/albums", get(artist_albums_handler))
        .route(
            "/artist/recommendations",
            get(recommend_artists_songs_handler),
        )
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
