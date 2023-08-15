use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::{
    models::{
        lyrics::Lyrics,
        response::{CustomResponse, StatusCode},
    },
    services::lyrics_service::get_lyrics,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
}

/// Handler for `/lyrics` route
///
/// ## Arguments
///
/// * `id` - song id
///
/// ## Returns
///
/// * `Json<CustomResponse<Lyrics>>` - Json response
pub async fn lyrics_handler(Query(params): Query<Params>) -> Json<CustomResponse<Lyrics>> {
    match params.id {
        Some(id) => {
            if id.is_empty() {
                return Json(CustomResponse::new(
                    StatusCode::Failed,
                    "❌ Song id is required!",
                    None,
                ));
            }

            let lyrics = get_lyrics(&id).await;

            let (status, message) = if lyrics.is_ok() {
                (StatusCode::Success, "✅ Lyrics fetched successfully!")
            } else {
                (StatusCode::Failed, "❌ Failed to fetch lyrics")
            };

            Json(CustomResponse::new(status, message, lyrics.ok()))
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Song id is required!",
            None,
        )),
    }
}
