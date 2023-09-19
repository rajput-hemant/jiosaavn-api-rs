use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        misc::Union,
        radio::{RRadioSongs, RRadioStation, RadioStationType},
        response::{CResponse, Status},
    },
    services::radio_service::{create_radio, get_radio_songs},
    utils::parse_bool,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioParams {
    pub song_id: Option<String>,
    pub artist_id: Option<String>,
    pub name: Option<String>,
    pub n: Option<String>,
    pub mode: Option<String>,
    pub lang: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
    pub raw: Option<String>,
    pub camel: Option<String>,
}

/// Handler for `/radio/(featured|artist|entity)` OR `/radio/create/(featured|artist|entity)` route
///
/// ## Arguments
///
/// * `name` - radio name
/// * `lang` - radio language
/// * `song_id` - song id
/// * `artist_id` - artist id
/// * `mode` - radio mode
/// * `id` - entity id
/// * `type` - entity type
/// * `raw` - weather to return raw response
/// * `camel` - weather to return camel case response keys
///
/// ## Returns
///
/// * `(StatusCode, Json<RRadioStation>)` - Json response
pub async fn create_radio_handler(
    Query(params): Query<RadioParams>,
    Path(path): Path<RadioStationType>,
) -> (StatusCode, Json<RRadioStation>) {
    let name = params.name.unwrap_or_default();
    let lang = params.lang.unwrap_or_default();
    let song_id = params.song_id.unwrap_or_default();
    let artist_id = params.artist_id.unwrap_or_default();
    let mode = params.mode.unwrap_or_default();
    let id = params.id.unwrap_or_default();
    let entity_type = params.entity_type.unwrap_or_default();
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    let (status, response) = match (path, name.is_empty(), id.is_empty()) {
        (RadioStationType::Featured | RadioStationType::Artist, true, _) => (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Radio Station Name is Required!".to_string(),
                None,
            )),
        ),
        (RadioStationType::Entity, _, true) => (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Entity ID is Required!".to_string(),
                None,
            )),
        ),
        (station_type, _, _) => {
            let radio = create_radio(
                song_id,
                artist_id,
                name,
                mode,
                lang,
                id,
                entity_type,
                station_type,
                raw,
                camel,
            )
            .await;

            match radio {
                Ok(radio) => (StatusCode::OK, radio),
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Union::B(CResponse::new(Status::Failed, e, None)),
                ),
            }
        }
    };

    (status, Json(response))
}

/// Handler for `/radio/songs` route
///
/// ## Arguments
///
/// * `id` - station id
/// * `n` - number of songs
/// * `raw` - weather to return raw response
/// * `camel` - weather to return camel case response keys
///
/// ## Returns
///
/// * `(StatusCode, Json<RRadioSongs>)` - Json response
pub async fn radio_songs_handler(
    Query(params): Query<RadioParams>,
) -> (StatusCode, Json<RRadioSongs>) {
    let id = params.id.unwrap_or_default();
    let n = params.n.unwrap_or("10".to_string());
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    let (status, response) = if id.is_empty() {
        (
            StatusCode::BAD_REQUEST,
            Union::B(CResponse::new(
                Status::Failed,
                "❌ Radio Station ID is Required!".to_string(),
                None,
            )),
        )
    } else {
        let radio = get_radio_songs(id, n, raw, camel).await;

        match radio {
            Ok(radio) => (StatusCode::OK, radio),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Union::B(CResponse::new(Status::Failed, e, None)),
            ),
        }
    };

    (status, Json(response))
}
