use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::{
    models::{
        album::AlbumResponse,
        response::{CustomResponse, StatusCode},
    },
    services::album_service::{
        get_album_details_by_id, get_album_details_by_link, get_album_recommendations,
    },
};

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    link: Option<String>,
}

/// Handler for `/album` route
///
/// ## Arguments
///
/// * `id` - Query parameter for album id
/// * `link` - Query parameter for album link
///
/// * In case both `id` and `link` are provided, `id` will be used
///
/// ## Returns
///
/// * `Json<CustomResponse<AlbumResponse>>` - Json response
pub async fn album_details_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<AlbumResponse>> {
    let (identifier, fetch_by_id) = match (params.id, params.link) {
        (Some(id), None) => (id, true),
        (None, Some(link)) => (link, false),
        (Some(id), Some(_)) => (id, true),
        (None, None) => {
            return Json(CustomResponse::new(
                StatusCode::Failed,
                "❌ Album id or link is required!",
                None,
            ));
        }
    };

    if identifier.is_empty() {
        return Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Album id or link is required!",
            None,
        ));
    }

    let album_result = if fetch_by_id {
        get_album_details_by_id(&identifier).await
    } else {
        get_album_details_by_link(&identifier).await
    };

    let (status, message) = if album_result.is_ok() {
        (
            StatusCode::Success,
            "✅ Album details fetched successfully!",
        )
    } else {
        (StatusCode::Failed, "❌ Failed to fetch album details!")
    };

    Json(CustomResponse::new(status, message, album_result.ok()))
}

/// Handler for `/album/recommendations` route
///
/// ## Arguments
///
/// * `id` - Query parameter for album id
///
/// ## Returns
///
/// * `Json<CustomResponse<Vec<AlbumResponse>>>` - Json response
pub async fn recommend_albums_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<Vec<AlbumResponse>>> {
    match params.id {
        Some(id) => {
            let result = get_album_recommendations(&id).await;

            let status = if result.is_ok() {
                StatusCode::Success
            } else {
                StatusCode::Failed
            };

            let message = if result.is_ok() {
                "✅ Successfully fetched album recommendations"
            } else {
                "❌ Failed to fetch album recommendations"
            };

            Json(CustomResponse::new(status, message, result.ok()))
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Album id is required!",
            None,
        )),
    }
}
