use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;

use crate::{
    models::{
        misc::Union,
        playlist::{RPlaylist, RPlaylistVec},
        response::{CResponse, Status},
    },
    services::playlist_service::{get_playlist_details, get_playlist_recommendations},
    utils::{is_jio_saavn_link, parse_bool, valid_langs},
};

#[derive(Debug, Deserialize)]
pub struct PlaylistParams {
    pub id: Option<String>,
    pub token: Option<String>,
    pub link: Option<String>,
    pub lang: Option<String>,
    pub raw: Option<String>,
    pub camel: Option<String>,
}

/// Handler for `/playlist` route
///
/// ## Arguments
///
/// * `id` - Query parameter for playlist id
/// * `token` - Query parameter for playlist token
/// * `link` - Query parameter for playlist link
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<PlaylistResponse>>)` - Status code and JSON response
pub async fn playlist_details_handler(
    Query(params): Query<PlaylistParams>,
) -> (StatusCode, Json<RPlaylist>) {
    let id = params.id.unwrap_or_default();
    let token = params.token.unwrap_or_default();
    let link = params.link.unwrap_or_default();
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    let (status, response) = match (id, token, link) {
        (id, token, link) if id.is_empty() && token.is_empty() && link.is_empty() => (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Please provide Playlist Id or token or a link".to_string(),
                None,
            )),
        ),

        (_, _, link)
            if !(link.is_empty() || is_jio_saavn_link(&link) && link.contains("featured")) =>
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
            let response = get_playlist_details(id, token, link, raw, camel).await;

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

/// Handler for `/playlist/recommend` route
/// 
/// ## Arguments
/// 
/// * `id` - Query parameter for playlist id
/// * `lang` - Optional query parameter for Comma separated languages
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
/// 
/// ## Returns
/// 
/// * `(StatusCode, Json<CResponse<RPlaylistVec>>)` - Status code and JSON response
pub async fn recommend_playlists_handler(
    Query(params): Query<PlaylistParams>,
) -> (StatusCode, Json<RPlaylistVec>) {
    let (status, response) = match params.id {
        Some(id) if !id.is_empty() => {
            let response = get_playlist_recommendations(
                id.to_string(),
                valid_langs(params.lang.unwrap_or_default()),
                parse_bool(params.raw.unwrap_or_default()),
                parse_bool(params.camel.unwrap_or_default()),
            )
            .await;

            match response {
                Ok(albums) => (StatusCode::OK, albums),

                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Union::B(CResponse::new(Status::Failed, e, None)),
                ),
            }
        }

        _ => (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Please provide a Playlist Id".to_string(),
                None,
            )),
        ),
    };

    (status, Json(response))
}
