use serde_json::Value;

use crate::{
    config::{CREATE_ARTIST_RADIO, CREATE_ENTITY_RADIO, CREATE_FEATURED_RADIO, RADIO_SONGS},
    models::{
        misc::Union,
        radio::{RRadioSongs, RRadioStation, RadioStationType},
        response::{CResponse, Status},
    },
    payloads::radio_payload::{radio_songs_payload, radio_station_payload},
    utils::formatted_payload,
};

use super::api_service::http;

/// Helper function to create radio station
///
/// ## Arguments
///
/// * `song_id` - song id
/// * `artist_id` - artist id
/// * `name` - radio name
/// * `mode` - radio mode
/// * `lang` - radio language
/// * `id` - entity id
/// * `entity_type` - entity type
/// * `station_type` - radio station type
/// * `raw` - weather to return raw response
/// * `camel` - weather to return camel case response keys
///
/// ## Returns
///
/// * `Result<RRadioStation, String>` - Result of radio station creation
#[allow(clippy::too_many_arguments)]
pub async fn create_radio(
    song_id: String,
    artist_id: String,
    name: String,
    mode: String,
    lang: String,
    id: String,
    entity_type: String,
    station_type: RadioStationType,
    raw: bool,
    camel: bool,
) -> Result<RRadioStation, String> {
    let path = match station_type {
        RadioStationType::Featured => CREATE_FEATURED_RADIO,
        RadioStationType::Artist => CREATE_ARTIST_RADIO,
        RadioStationType::Entity => CREATE_ENTITY_RADIO,
    };

    let query = match station_type {
        RadioStationType::Featured => vec![
            ("name".to_string(), name.clone()),
            ("query".to_string(), name),
            ("song_id".to_string(), song_id),
            ("artist_id".to_string(), artist_id),
            ("mode".to_string(), mode),
            ("language".to_string(), lang),
        ]
        .into_iter()
        .collect(),

        RadioStationType::Artist => vec![
            ("name".to_string(), name.clone()),
            ("query".to_string(), name),
            ("song_id".to_string(), song_id),
            ("artist_id".to_string(), artist_id),
            ("mode".to_string(), mode),
            ("language".to_string(), lang),
        ]
        .into_iter()
        .collect(),

        RadioStationType::Entity => vec![
            ("entity_id".to_string(), id),
            ("entity_type".to_string(), format!("{:?}", entity_type)),
        ]
        .into_iter()
        .collect(),
    };

    let response = http::<Value>(path, true, Some(query)).await;

    match response {
        Ok(radio) => {
            if radio.get("error").is_some() {
                if radio["error"].is_string() {
                    Err(radio["error"].as_str().unwrap().to_string())
                } else {
                    Err(radio["error"]["msg"].as_str().unwrap().to_string())
                }
            } else if raw {
                Ok(Union::A(radio))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    format!("✅ Successfully created {:?} Radio station", station_type),
                    Some(formatted_payload(radio, camel, &radio_station_payload)),
                )))
            }
        }
        Err(e) => {
            println!("Error: {e}");

            Err(format!(
                "❌ Unable to create {:?} Radio station",
                station_type
            ))
        }
    }
}

/// Helper function to get radio station songs
///
/// ## Arguments
///
/// * `station_id` - radio station id
///
/// ## Returns
///
/// * `Result<RRadioSongs, String>` - Result of radio station songs
pub async fn get_radio_songs(
    station_id: String,
    n: String,
    raw: bool,
    camel: bool,
) -> Result<RRadioSongs, String> {
    let query = vec![("stationid".to_string(), station_id), ("k".to_string(), n)]
        .into_iter()
        .collect();

    let response = http::<Value>(RADIO_SONGS, true, Some(query)).await;

    match response {
        Ok(radio) => {
            if radio.get("error").is_some() {
                Err(radio["error"].as_str().unwrap().to_string())
            } else if raw {
                Ok(Union::A(radio))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Radio Station Songs Fetched Successfully!".to_string(),
                    Some(formatted_payload(radio, camel, &radio_songs_payload)),
                )))
            }
        }
        Err(e) => {
            println!("Error: {e}");

            Err("❌ Unable to fetch radio station songs".to_string())
        }
    }
}
