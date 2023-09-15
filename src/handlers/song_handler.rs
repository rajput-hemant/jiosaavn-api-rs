use crate::{
    models::{
        misc::Union,
        response::{CResponse, Status},
        song::{RSong, RSongReco},
    },
    services::song_service::{get_song_details, get_song_recommendations},
    utils::{is_jio_saavn_link, parse_bool},
};
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SongParams {
    pub id: Option<String>,
    pub token: Option<String>,
    pub link: Option<String>,
    pub raw: Option<String>,
    pub camel: Option<String>,
    pub lang: Option<String>,
}

/// Handler for `/song` route
///
/// ## Arguments
///
/// * `id` - Query param for song(s) id(s)
/// * `token` - Query param for song token
/// * `link` - Query param for song link
///
/// ## Returns
///
/// * `Json<RSong>` - Json response
pub async fn song_details_handler(Query(params): Query<SongParams>) -> Json<RSong> {
    match (params.id, params.token, params.link) {
        (None, None, None) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide song id(s) or token or a link",
            None,
        ))),

        (_, _, Some(link)) if !is_jio_saavn_link(link.to_owned()) => {
            Json(Union::B(CResponse::new(
                Status::Failed,
                "❌ Please provide a valid JioSaavn link",
                None,
            )))
        }

        (id, token, link) => Json(
            get_song_details(
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

/// Handler for `/song/recommendations` route
///
/// ## Arguments
///
/// * `id` - Query parameter for song id
///
/// ## Returns
///
/// * `Json<CustomResponse<Vec<SongResponse>>>` - Json response
pub async fn recommend_songs_handler(Query(params): Query<SongParams>) -> Json<RSongReco> {
    match (params.id, params.lang) {
        (Some(id), lang) => Json(
            get_song_recommendations(
                id,
                lang.unwrap_or_else(|| "hindi,english".to_string()),
                parse_bool(params.raw.unwrap_or_else(|| "0".to_string())),
                parse_bool(params.camel.unwrap_or_else(|| "0".to_string())),
            )
            .await,
        ),
        (None, _) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide song id",
            None,
        ))),
    }
}
