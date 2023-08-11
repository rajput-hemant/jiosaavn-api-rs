use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use reqwest::Error;

use crate::{
    models::radio::{Radio, RadioSongResponse},
    payloads::radio_payload::radio_song_payload,
};

use super::api_service::http;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RadioStationType {
    Featured,
    Artist,
    Entity,
}

/// Helper function to make request to `webradio.createFeaturedStation` endpoint of JioSaavn API to create radio
///
/// ## Arguments
///
/// * `name` - Name of the radio
/// * `language` - Language of the radio
/// * `station_type` - Type of the radio
///
/// ## Returns
///
/// * `Result<Radio, Error>` - Result of radio payload
pub async fn create_radio(
    name: Vec<&str>,
    language: &str,
    station_type: RadioStationType,
) -> Result<Radio, Error> {
    let endpoint = match station_type {
        RadioStationType::Featured => "webradio.createFeaturedStation",
        RadioStationType::Artist => "webradio.createArtistStation",
        RadioStationType::Entity => "webradio.createEntityStation",
    };

    let params: Option<HashMap<String, String>> = match station_type {
        RadioStationType::Featured => Some(
            vec![
                ("name".to_string(), name[0].to_string()),
                ("language".to_string(), language.to_string()),
            ]
            .into_iter()
            .collect(),
        ),
        RadioStationType::Artist => Some(
            vec![
                ("name".to_string(), name[0].to_string()),
                ("query".to_string(), name[0].to_string()),
                ("language".to_string(), language.to_string()),
            ]
            .into_iter()
            .collect(),
        ),
        RadioStationType::Entity => Some(
            vec![
                (
                    "entity_id".to_string(),
                    name.iter()
                        .map(|e| format!("\"{e}\""))
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                ("entity_type".to_string(), "queue".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    };

    let result = http(endpoint, true, params).await?;

    Ok(result)
}

/// Helper function to make request to `webradio.getSong` endpoint of JioSaavn API to get radio songs
///
/// ## Arguments
///
/// * `station_id` - Station id
/// * `count` - Count of songs to fetch
/// * `next` - Next index
///
/// ## Returns
///
/// * `Result<RadioSongResponse, Error>` - Result of radio song payload
pub async fn get_radio_songs(
    station_id: &str,
    count: u64,
    next: u64,
) -> Result<RadioSongResponse, Error> {
    let params: Option<HashMap<String, String>> = Some(
        vec![
            ("stationid".to_string(), station_id.to_string()),
            ("k".to_string(), count.to_string()),
            ("next".to_string(), next.to_string()),
        ]
        .into_iter()
        .collect(),
    );

    let result = http("webradio.getSong", true, params).await?;

    Ok(radio_song_payload(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_radio() -> Result<(), Error> {
        let result = create_radio(vec!["Arijit Singh"], "hindi", RadioStationType::Artist).await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_radio_songs() -> Result<(), Error> {
        let result = get_radio_songs(
            "z4jVxXCk2U70olZbGArPKmAINUg2-4NRNs1JimLO9Cgb5pKgepaRvA__~^~artist_radio~^~459320",
            10,
            0,
        )
        .await?;

        dbg!(result);

        Ok(())
    }
}
