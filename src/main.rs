mod handlers;
mod models;
mod payloads;
mod services;
mod utils;

use axum::{
    error_handling::HandleErrorLayer,
    http::{Method, StatusCode},
    response::Html,
    routing::get,
    BoxError, Router,
};
use dotenv::dotenv;
use std::{env, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};

use handlers::{
    album_details_handler, albums_search_handler, artist_albums_handler, artist_details_handler,
    artist_songs_handler, artists_search_handler, create_radio_handler, lyrics_handler,
    modules_handler, playlist_details_handler, playlists_search_handler, radio_songs_handler,
    recommend_albums_handler, recommend_artists_songs_handler, recommend_songs_handler,
    search_all_handler, song_details_handler, songs_search_handler, top_searches_handler,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let enable_rate_limit = env::var("ENABLE_RATE_LIMIT")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false);

    let mut router = Router::new()
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
        // search routes
        .route("/search", get(search_all_handler))
        .route("/search/all", get(search_all_handler))
        .route("/search/top", get(top_searches_handler))
        .route("/search/songs", get(songs_search_handler))
        .route("/search/albums", get(albums_search_handler))
        .route("/search/artists", get(artists_search_handler))
        .route("/search/playlists", get(playlists_search_handler))
        // radio routes
        .route("/radio", get(create_radio_handler))
        .route("/radio/songs", get(radio_songs_handler))
        // lyrics route
        .route("/lyrics", get(lyrics_handler))
        // cors layer
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        );

    // rate limit layer
    // https://github.com/tokio-rs/axum/discussions/987#discussioncomment-2678595
    if enable_rate_limit {
        router = router.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
        );
    }

    let addr = "[::]:8080".parse().unwrap();

    tracing::debug!("🚀 Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
}
