use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    models::{
        artist::{RArtist, RArtistSongsOrAlbums, RArtistTopSongs},
        misc::Union,
        response::{CResponse, Status},
    },
    services::{get_artist_top_songs, get_artists_details, get_artists_songs_albums},
    utils::{is_jio_saavn_link, parse_bool},
};

#[derive(Debug, Deserialize)]
pub struct ArtistParams {
    pub id: Option<String>,
    pub link: Option<String>,
    pub token: Option<String>,
    pub song_id: Option<String>,
    pub artist_id: Option<String>,
    pub page: Option<String>,
    pub n_song: Option<String>,
    pub n_album: Option<String>,
    pub cat: Option<String>,
    pub sort: Option<String>,
    pub lang: Option<String>,
    pub raw: Option<String>,
    pub camel: Option<String>,
}

/// Handler for `/artist` route
///
/// ## Arguments
///
/// * `id` - Query parameter for artist id
/// * `link` - Query parameter for artist link
/// * `token` - Query parameter for artist token
/// * `page` - Query parameter for page number
/// * `n_song` - Query parameter for number of songs
/// * `n_album` - Query parameter for number of albums
/// * `raw` - Query parameter for raw response
/// * `camel` - Query parameter for camel case response
///
/// ## Returns
///
/// * `(StatusCode, Json<RArtist>` - Json response
pub async fn artist_details_handler(
    Query(params): Query<ArtistParams>,
) -> (StatusCode, Json<RArtist>) {
    let id = params.id.unwrap_or_default();
    let token = params.token.unwrap_or_default();
    let link = params.link.unwrap_or_default();
    let page = params.page.unwrap_or_default();
    let n_song = params.n_song.unwrap_or("10".to_string());
    let n_album = params.n_album.unwrap_or("10".to_string());
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    let (status, resposne) = match (id, token, link) {
        (id, token, link) if id.is_empty() && token.is_empty() && link.is_empty() => (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Please provide Artist Id, link or token".to_string(),
                None,
            )),
        ),

        (_, _, link)
            if !(link.is_empty() || is_jio_saavn_link(&link) && link.contains("artist")) =>
        {
            (
                StatusCode::BAD_REQUEST,
                Union::B(CResponse::new(
                    Status::Failed,
                    "❌ Please provide a valid JioSaavn link".to_string(),
                    None,
                )),
            )
        }

        (id, token, link) => {
            let response =
                get_artists_details(id, token, link, page, n_song, n_album, raw, camel).await;

            match response {
                Ok(song) => (StatusCode::OK, song),
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Union::B(CResponse::new(Status::Failed, e, None)),
                ),
            }
        }
    };

    (status, Json(resposne))
}

/// Handler for `/artist/songs` or `/artist/albums` route
///
/// ## Arguments
///
/// * `id` - Path parameter for artist id
/// * `page` - Query parameter for page number
/// * `cat` - Query parameter for category
/// * `sort` - Query parameter for sort
/// * `raw` - Query parameter for raw response
/// * `camel` - Query parameter for camel case response
///
/// ## Returns
///
/// * `(StatusCode, Json<RArtistSongsOrAlbums>)` - Json response
pub async fn artist_songs_albums_handler(
    Query(params): Query<ArtistParams>,
    Path(path): Path<String>,
) -> (StatusCode, Json<RArtistSongsOrAlbums>) {
    let id = params.id.unwrap_or_default();
    let page = params.page.unwrap_or_default();
    let cat = params.cat.unwrap_or_default();
    let sort = params.sort.unwrap_or_default();
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    let (status, response) = match id {
        id if id.is_empty() => (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Please provide Artist Id".to_string(),
                None,
            )),
        ),

        id => {
            let response = get_artists_songs_albums(id, page, cat, sort, path, raw, camel).await;

            match response {
                Ok(song) => (StatusCode::OK, song),
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Union::B(CResponse::new(Status::Failed, e, None)),
                ),
            }
        }
    };

    (status, Json(response))
}

/// Handler for `/artist/recommend` route
///
/// ## Arguments
///
/// * `artist_id` - Query parameter for artist id
/// * `song_id` - Query parameter for song id
/// * `lang` - Query parameter for language
/// * `page` - Query parameter for page number
/// * `cat` - Query parameter for category
/// * `sort` - Query parameter for sort
/// * `raw` - Query parameter for raw response
/// * `camel` - Query parameter for camel case response
///
/// ## Returns
///
/// * `(StatusCode, Json<RArtistTopSongs>)` - Json response
pub async fn recommend_artists_songs_handler(
    Query(params): Query<ArtistParams>,
) -> (StatusCode, Json<RArtistTopSongs>) {
    let artist_id = params.artist_id.unwrap_or_default();
    let song_id = params.song_id.unwrap_or_default();
    let lang = params.lang.unwrap_or_default();
    let page = params.page.unwrap_or_default();
    let cat = params.cat.unwrap_or_default();
    let sort = params.sort.unwrap_or_default();
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    let (status, response) = match (artist_id, song_id) {
        (artist_id, song_id) if artist_id.is_empty() || song_id.is_empty() => {
            let a = if artist_id.is_empty() {
                "Artist"
            } else {
                "Song"
            };

            (
                StatusCode::BAD_REQUEST,
                Union::B(CResponse::new(
                    Status::Failed,
                    format!("❌ Please provide {a} Id"),
                    None,
                )),
            )
        }

        (artist_id, song_id) => {
            let response =
                get_artist_top_songs(artist_id, song_id, page, lang, cat, sort, raw, camel).await;

            match response {
                Ok(song) => (StatusCode::OK, song),
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Union::B(CResponse::new(Status::Failed, e, None)),
                ),
            }
        }
    };

    (status, Json(response))
}
