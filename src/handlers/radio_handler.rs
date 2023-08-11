use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        radio::{Radio, RadioSongResponse},
        response::{CustomResponse, StatusCode},
    },
    services::radio_service::{create_radio, get_radio_songs, RadioStationType},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    name: Option<String>,
    language: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<RadioStationType>,

    id: Option<String>,
    count: Option<u64>,
    next: Option<u64>,
}

/// Handler for `/radio` OR `/radio/create` route
///
/// ## Arguments
///
/// * `name` - radio name
/// * `language` - radio language
/// * `type` - radio type
///
/// ## Returns
///
/// * `Json<CustomResponse<Radio>>` - Json response
pub async fn create_radio_handler(Query(params): Query<Params>) -> Json<CustomResponse<Radio>> {
    let (name, language, station_type) = (params.name, params.language, params.type_field);

    match (name, station_type) {
        (Some(name), Some(station_type)) => {
            let result = create_radio(
                name.split(",").collect(),
                &language.unwrap_or("hindi,english".to_string()),
                station_type,
            )
            .await;

            match result {
                Ok(radio) => Json(CustomResponse {
                    status: StatusCode::Success,
                    message: "✅ Radio created successfully",
                    data: Some(radio),
                }),
                Err(_) => Json(CustomResponse {
                    status: StatusCode::Failed,
                    message: "❌ Failed to create radio",
                    data: None,
                }),
            }
        }
        _ => Json(CustomResponse {
            status: StatusCode::Failed,
            message: "❌ Missing required parameters",
            data: None,
        }),
    }
}

/// Handler for `/radio/songs` route
///
/// ## Arguments
///
/// * `id` - station id
/// * `count` - count of songs to fetch
/// * `next` - next index
///
/// ## Returns
///
/// * `Json<CustomResponse<RadioSongResponse>>` - Json response
pub async fn radio_songs_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<RadioSongResponse>> {
    let (id, count, next) = (params.id, params.count, params.next);

    match id {
        Some(id) => {
            let result = get_radio_songs(&id, count.unwrap_or(10), next.unwrap_or(0)).await;

            match result {
                Ok(radio) => Json(CustomResponse {
                    status: StatusCode::Success,
                    message: "✅ Radio songs fetched successfully",
                    data: Some(radio),
                }),
                Err(_) => Json(CustomResponse {
                    status: StatusCode::Failed,
                    message: "❌ Failed to fetch radio songs",
                    data: None,
                }),
            }
        }
        _ => Json(CustomResponse {
            status: StatusCode::Failed,
            message: "❌ Station id is required",
            data: None,
        }),
    }
}
