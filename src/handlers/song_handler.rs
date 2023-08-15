use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::{
    models::{
        response::{CustomResponse, StatusCode},
        song::SongResponse,
    },
    services::song_service::{
        get_song_details_by_id, get_song_details_by_link, get_song_recommendations,
    },
};

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    link: Option<String>,
}

/// Handler for `/song` route
///
/// ## Arguments
///
/// * `id` - Query parameter for song id
/// * `link` - Query parameter for song link
///
/// * In case both `id` and `link` are provided, `id` will be used
///
/// ## Returns
///
/// * `Json<CustomResponse<Vec<SongResponse>>>` - Json response
pub async fn song_details_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<Vec<SongResponse>>> {
    let (identifier, fetch_by_id) = match (params.id, params.link) {
        (Some(id), None) => (id, true),
        (None, Some(link)) => (link, false),
        (Some(id), Some(_)) => (id, true),
        (None, None) => {
            return Json(CustomResponse::new(
                StatusCode::Failed,
                "❌ Song id or link is required!",
                None,
            ));
        }
    };

    if identifier.is_empty() {
        return Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Song id or link is required!",
            None,
        ));
    }

    let song_result = if fetch_by_id {
        get_song_details_by_id(&identifier).await
    } else {
        get_song_details_by_link(&identifier).await
    };

    let (status, message) = if song_result.is_ok() {
        (StatusCode::Success, "✅ Song details fetched successfully!")
    } else {
        (StatusCode::Failed, "❌ Failed to fetch song details!")
    };

    Json(CustomResponse::new(status, message, song_result.ok()))
}

/// Handler for `/song/recommendations` route
///
/// ## Arguments
///
/// * `id` - Query parameter for song id
///
/// ## Returns
///
/// * `Json<CustomResponse<Vec<SongResponse>>>` - Json response
pub async fn recommend_songs_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<Vec<SongResponse>>> {
    match params.id {
        Some(id) => {
            let result = get_song_recommendations(&id).await;

            let status = if result.is_ok() {
                StatusCode::Success
            } else {
                StatusCode::Failed
            };

            let message = if result.is_ok() {
                "✅ Successfully fetched song recommendations"
            } else {
                "❌ Failed to fetch song recommendations"
            };

            Json(CustomResponse::new(status, message, result.ok()))
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Song id is required!",
            None,
        )),
    }
}
