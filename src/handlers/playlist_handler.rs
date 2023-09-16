use crate::{
    models::{
        misc::Union,
        playlist::{RPlaylist, RPlaylistRec},
        response::{CResponse, Status},
    },
    services::playlist_service::{get_playlist_details, get_playlist_recommendations},
    utils::{is_jio_saavn_link, parse_bool},
};
use axum::{extract::Query, Json};
use serde::Deserialize;
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
///
/// ## Returns
///
/// * `Json<CustomResponse<PlaylistResponse>>` - Json response
pub async fn playlist_details_handler(Query(params): Query<PlaylistParams>) -> Json<RPlaylist> {
    match (params.id, params.token, params.link) {
        (None, None, None) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide playlist id or token or a link",
            None,
        ))),

        (_, _, Some(link)) if !is_jio_saavn_link(link.to_owned()) && link.contains("featured") => {
            Json(Union::B(CResponse::new(
                Status::Failed,
                "❌ Please provide a valid JioSaavn link",
                None,
            )))
        }

        (id, token, link) => Json(
            get_playlist_details(
                id.unwrap_or_default(),
                token.unwrap_or_default(),
                link.unwrap_or_default(),
                parse_bool(params.raw.unwrap_or_else(|| "0".to_string())),
                parse_bool(params.camel.unwrap_or_else(|| "0".to_string())),
            )
            .await,
        ),
    }
}

pub async fn recommend_playlists_handler(
    Query(params): Query<PlaylistParams>,
) -> Json<RPlaylistRec> {
    match (params.id, params.lang) {
        (Some(id), lang) => Json(
            get_playlist_recommendations(
                id,
                lang.unwrap_or_default(),
                parse_bool(params.raw.unwrap_or_else(|| "0".to_string())),
                parse_bool(params.camel.unwrap_or_else(|| "0".to_string())),
            )
            .await,
        ),
        (None, _) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide playlist id",
            None,
        ))),
    }
}
