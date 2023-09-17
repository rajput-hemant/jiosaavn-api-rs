use crate::{
    models::{
        misc::Union,
        response::{CResponse, Status},
        song::{RSong, RSongReco},
    },
    services::song_service::{get_song_details, get_song_recommendations},
    utils::{is_jio_saavn_link, parse_bool, valid_langs},
};
use axum::{extract::Query, http::StatusCode, Json};
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
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<SongResponse>>)` - Status code and JSON response
pub async fn song_details_handler(Query(params): Query<SongParams>) -> (StatusCode, Json<RSong>) {
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
                "❌ Please provide Song Id(s) or token or a link".to_string(),
                None,
            )),
        ),

        (_, _, link) if !(link.is_empty() || is_jio_saavn_link(&link) && link.contains("song")) => {
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
            let response = get_song_details(id, token, link, raw, camel).await;

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

/// Handler for `/song/recommend` route
///
/// ## Arguments
///
/// * `id` - Query parameter for song id
/// * `lang` - Query parameter for Comma separated languages
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<RSongReco>>)` - Status code and JSON response
pub async fn recommend_songs_handler(
    Query(params): Query<SongParams>,
) -> (StatusCode, Json<RSongReco>) {
    let (status, response) = match params.id {
        Some(id) if !id.is_empty() => {
            let response = get_song_recommendations(
                id.to_string(),
                valid_langs(params.lang.unwrap_or_default()),
                parse_bool(params.raw.unwrap_or_default()),
                parse_bool(params.camel.unwrap_or_default()),
            )
            .await;

            match response {
                Ok(songs) => (StatusCode::OK, songs),

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
                "❌ Please provide a song id".to_string(),
                None,
            )),
        ),
    };

    (status, Json(response))
}
