use axum::{extract::Query, Json};
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
    utils::{is_jio_saavn_link, parse_bool},
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
///
/// ## Returns
///
/// * `Json<RAlbum>` - Json response
pub async fn album_details_handler(Query(params): Query<AlbumParams>) -> Json<RAlbum> {
    match (params.id, params.token, params.link) {
        (None, None, None) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide album id or token or a link",
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
            get_album_details(
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

/// Handler for `/album/recommendations` route
///
/// ## Arguments
///
/// * `id` - Query parameter for album id
/// * `lang` - Query parameter for album language
///
/// ## Returns
///
/// * `Json<RAlbumVec>` - Json response
pub async fn recommend_albums_handler(Query(params): Query<AlbumParams>) -> Json<RAlbumVec> {
    match (params.id, params.lang) {
        (Some(id), lang) => Json(
            get_album_recommendations(
                id,
                lang.unwrap_or_else(|| "hindi,english".to_string()),
                parse_bool(params.raw.unwrap_or_else(|| "0".to_string())),
                parse_bool(params.camel.unwrap_or_else(|| "0".to_string())),
            )
            .await,
        ),
        (None, _) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide album id",
            None,
        ))),
    }
}

/// Handler for `/album/same-year` route
///
/// ## Arguments
///
/// * `year` - Query parameter for album year
/// * `lang` - Query parameter for album language
///
/// ## Returns
///
/// * `Json<RAlbumVec>` - Json response
pub async fn albums_from_same_year_handler(Query(params): Query<AlbumParams>) -> Json<RAlbumVec> {
    match (params.year, params.lang) {
        (Some(year), lang) => Json(
            get_album_from_same_year(
                year,
                lang.unwrap_or_default(),
                parse_bool(params.raw.unwrap_or_else(|| "0".to_string())),
                parse_bool(params.camel.unwrap_or_else(|| "0".to_string())),
            )
            .await,
        ),
        (None, _) => Json(Union::B(CResponse::new(
            Status::Failed,
            "❌ Please provide album year",
            None,
        ))),
    }
}
