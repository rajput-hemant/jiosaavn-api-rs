use std::collections::HashMap;

use super::api_service::http;

pub enum RadioStationType {
    Featured,
    Artist,
    Entity,
}

pub async fn create_radio(
    name: Vec<&str>,
    language: &str,
    station_type: RadioStationType,
) -> Result<serde_json::Value, reqwest::Error> {
    let endpoint = match station_type {
        RadioStationType::Featured => "webradio.createFeaturedStation",
        RadioStationType::Artist => "webradio.createArtistStation",
        RadioStationType::Entity => "entitywebradio.createEntityStation",
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

    let result: serde_json::Value = http(endpoint, true, params).await?;

    Ok(result)
}

pub async fn get_radio_songs(
    station_id: &str,
    count: i64,
    next: i64,
) -> Result<serde_json::Value, reqwest::Error> {
    let params: Option<HashMap<String, String>> = Some(
        vec![
            ("stationid".to_string(), station_id.to_string()),
            ("k".to_string(), count.to_string()),
            ("next".to_string(), next.to_string()),
        ]
        .into_iter()
        .collect(),
    );

    let result: serde_json::Value = http("webradio.getSong", true, params).await?;

    Ok(result)
}
