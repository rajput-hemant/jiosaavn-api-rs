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
    id: String,
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
    let lyrics = get_lyrics(&params.id).await;

    let status = if lyrics.is_ok() {
        StatusCode::Success
    } else {
        StatusCode::Failed
    };

    let message = if lyrics.is_ok() {
        "✅ Lyrics fetched successfully!"
    } else {
        "❌ Failed to fetch lyrics!"
    };

    Json(CustomResponse::new(status, message, lyrics.ok()))
}
