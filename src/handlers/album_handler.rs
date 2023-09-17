use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;

use crate::{
    models::{
        album::{RAlbum, RAlbumVec},
        misc::Union,
        response::{CResponse, Status},
    },
    services::album_service::{
        get_album_details, get_album_from_same_year, get_album_recommendations,
    },
    utils::{is_jio_saavn_link, parse_bool, valid_langs},
};

#[derive(Debug, Deserialize)]
pub struct AlbumParams {
    pub id: Option<String>,
    pub token: Option<String>,
    pub link: Option<String>,
    pub raw: Option<String>,
    pub lang: Option<String>,
    pub year: Option<String>,
    pub camel: Option<String>,
}

/// Handler for `/album` route
///
/// ## Arguments
///
/// * `id` - Query parameter for album id
/// * `token` - Query parameter for album token
/// * `link` - Query parameter for album link
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<RAlbum>>)` - Status code and JSON response
pub async fn album_details_handler(
    Query(params): Query<AlbumParams>,
) -> (StatusCode, Json<RAlbum>) {
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
                "❌ Please provide Album Id or token or a link".to_string(),
                None,
            )),
        ),

        (_, _, link)
            if !(link.is_empty() || is_jio_saavn_link(&link) && link.contains("album")) =>
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
            let response = get_album_details(id, token, link, raw, camel).await;

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

/// Handler for `/album/recommend` route
///
/// ## Arguments
///
/// * `id` - Query parameter for album id
/// * `lang` - Query parameter for album language
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<RAlbumVec>>)` - Status code and JSON response
pub async fn recommend_albums_handler(
    Query(params): Query<AlbumParams>,
) -> (StatusCode, Json<RAlbumVec>) {
    let (status, response) = match params.id {
        Some(id) if !id.is_empty() => {
            let response = get_album_recommendations(
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
                "❌ Please provide a Album Id".to_string(),
                None,
            )),
        ),
    };

    (status, Json(response))
}

/// Handler for `/album/same-year` route
///
/// ## Arguments
///
/// * `year` - Query parameter for album year
/// * `lang` - Query parameter for album language
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<RAlbumVec>>)` - Status code and JSON response
pub async fn albums_from_same_year_handler(
    Query(params): Query<AlbumParams>,
) -> (StatusCode, Json<RAlbumVec>) {
    let (status, response) = match params.year {
        Some(year) if !year.is_empty() => {
            let response = get_album_from_same_year(
                year.to_string(),
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
                "❌ Please provide a Album Id".to_string(),
                None,
            )),
        ),
    };

    (status, Json(response))
}
