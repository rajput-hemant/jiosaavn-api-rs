use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::{
    models::{
        artist::{ArtistAlbumsResponse, ArtistResponse, ArtistSongsResponse},
        response::{CustomResponse, StatusCode},
        song::SongResponse,
    },
    services::artist_service::{
        get_artist_albums, get_artist_details_by_id, get_artist_details_by_link, get_artist_songs,
        get_artist_top_songs,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    id: Option<String>,
    link: Option<String>,
    song_id: Option<String>,
    artist_id: Option<String>,
    page: Option<u64>,
    category: Option<String>,
    sort: Option<String>,
    language: Option<String>,
}

/// Handler for `/artist` route
///
/// ## Arguments
///
/// * `id` - Query parameter for artist id
/// * `link` - Query parameter for artist link
///
/// * In case both `id` and `link` are provided, `id` will be used
///
/// ## Returns
///
/// * `Json<CustomResponse<ArtistResponse>>` - Json response
pub async fn artist_details_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<ArtistResponse>> {
    let (identifier, fetch_by_id) = match (params.id, params.link) {
        (Some(id), None) => (id, true),
        (None, Some(link)) => (link, false),
        (Some(id), Some(_)) => (id, true),
        (None, None) => {
            return Json(CustomResponse::new(
                StatusCode::Failed,
                "❌ Artist id or link is required!",
                None,
            ));
        }
    };

    let artist_result = if fetch_by_id {
        get_artist_details_by_id(&identifier).await
    } else {
        get_artist_details_by_link(&identifier).await
    };

    // println!("{}", artist_result.as_ref().unwrap());

    let status = if artist_result.is_ok() {
        StatusCode::Success
    } else {
        StatusCode::Failed
    };

    let message = if artist_result.is_ok() {
        "✅ Artist details fetched successfully!"
    } else {
        "❌ Failed to fetch artist details!"
    };

    Json(CustomResponse::new(status, message, artist_result.ok()))
}

/// Handler for `/artist/songs` route
///
/// ## Arguments
///
/// * `id` - Path parameter for artist id
/// * `page` - Query parameter for page number
/// * `category` - Query parameter for category
/// * `sort` - Query parameter for sort
///
/// ## Returns
///
/// * `Json<CustomResponse<ArtistSongsResponse>>` - Json response
pub async fn artist_songs_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<ArtistSongsResponse>> {
    match params.id {
        Some(id) => {
            let result = get_artist_songs(
                &id,
                params.page.unwrap_or(1),
                &params.category.unwrap_or("latest".to_string()),
                &params.sort.unwrap_or("asc".to_string()),
            )
            .await;

            let status = if result.is_ok() {
                StatusCode::Success
            } else {
                StatusCode::Failed
            };

            let message = if result.is_ok() {
                "✅ Artist songs fetched successfully!"
            } else {
                "❌ Failed to fetch artist songs!"
            };

            Json(CustomResponse::new(status, message, result.ok()))
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Artist id is required!",
            None,
        )),
    }
}

/// Handler for `/artist/albums` route
///
/// ## Arguments
///
/// * `id` - Path parameter for artist id
/// * `page` - Query parameter for page number
/// * `category` - Query parameter for category
/// * `sort` - Query parameter for sort
///
/// ## Returns
///
/// * `Json<CustomResponse<ArtistAlbumsResponse>>` - Json response
pub async fn artist_albums_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<ArtistAlbumsResponse>> {
    match params.id {
        Some(id) => {
            let result = get_artist_albums(
                &id,
                params.page.unwrap_or(1),
                &params.category.unwrap_or("latest".to_string()),
                &params.sort.unwrap_or("asc".to_string()),
            )
            .await;

            let status = if result.is_ok() {
                StatusCode::Success
            } else {
                StatusCode::Failed
            };

            let message = if result.is_ok() {
                "✅ Artist albums fetched successfully!"
            } else {
                "❌ Failed to fetch artist albums!"
            };

            Json(CustomResponse::new(status, message, result.ok()))
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Artist id is required!",
            None,
        )),
    }
}

/// Handler for `/artist/recommendations` route
///
/// ## Arguments
///
/// * `artist_id` - Query parameter for artist id
/// * `song_id` - Query parameter for song id
/// * `language` - Query parameter for language
///
/// ## Returns
///
/// * `Json<CustomResponse<Vec<SongResponse>>>` - Json response
pub async fn recommend_artists_songs_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<Vec<SongResponse>>> {
    match (params.artist_id, params.song_id) {
        (Some(artist_id), Some(song_id)) => {
            let result = get_artist_top_songs(
                &artist_id,
                &song_id,
                &params.language.unwrap_or("english".to_string()),
            )
            .await;

            let status = if result.is_ok() {
                StatusCode::Success
            } else {
                StatusCode::Failed
            };

            let message = if result.is_ok() {
                "✅ Recommended artists songs fetched successfully!"
            } else {
                "❌ Failed to fetch recommended artists songs!"
            };

            Json(CustomResponse::new(status, message, result.ok()))
        }
        (None, Some(_)) => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Artist id is required!",
            None,
        )),
        (Some(_), None) => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Song id is required!",
            None,
        )),
        (None, None) => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Artist id and song id is required!",
            None,
        )),
    }
}
