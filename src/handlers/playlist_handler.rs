use crate::{
    models::{
        playlist::PlaylistResponse,
        response::{CustomResponse, StatusCode},
    },
    services::playlist_service::get_playlist_details_by_id,
};
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    pub id: Option<String>,
}

/// Handler for `/playlist` route
///
/// ## Arguments
///
/// * `id` - Query parameter for playlist id
///
/// ## Returns
///
/// * `Json<CustomResponse<PlaylistResponse>>` - Json response
pub async fn playlist_details_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<PlaylistResponse>> {
    let id = match params.id {
        Some(id) => id,
        None => {
            return Json(CustomResponse::new(
                StatusCode::Failed,
                "❌ Playlist id is required!",
                None,
            ));
        }
    };

    if id.is_empty() {
        return Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Playlist id is required!",
            None,
        ));
    }

    let playlist_result = get_playlist_details_by_id(&id).await;

    Json(CustomResponse {
        status: if playlist_result.is_ok() {
            StatusCode::Success
        } else {
            StatusCode::Failed
        },
        message: if playlist_result.is_ok() {
            "✅ Playlist details fetched successfully!"
        } else {
            "❌ Failed to fetch playlist details!"
        },

        data: playlist_result.ok(),
    })
}
