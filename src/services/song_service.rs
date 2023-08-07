use serde_json::{from_value, Value};

use crate::{
    models::song::{SongRequest, SongResponse},
    payloads::song_payload,
};

use super::api_service::http;

/// Helper function to make request to `song.getDetails` endpoint of JioSaavn API to get song details
///
/// ## Arguments
///
/// * `ids` - Comma separated song ids
///
/// ## Returns
///
/// * `Result<Vec<SongResponse>, reqwest::Error>` - Result of song payload
pub async fn get_song_details_by_id(ids: &str) -> Result<Vec<SongResponse>, reqwest::Error> {
    let result: Value = http(
        "song.getDetails",
        true,
        Some(
            vec![("pids".to_string(), ids.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    let songs = parse_song_value(result)
        .into_iter()
        .map(|song| song_payload(song))
        .collect();

    Ok(songs)
}

/// Helper function to make request to `webapi.get` endpoint of JioSaavn API to get song details
///
/// ## Arguments
///
/// * `link` - Song link
///
/// ## Returns
///
/// * `Result<Vec<SongResponse>, reqwest::Error>` - Result of song payload
///
/// ## _Note_
///
/// * This function is not recommended to use as it is not stable
pub async fn get_song_details_by_link(link: &str) -> Result<Vec<SongResponse>, reqwest::Error> {
    let result: Value = http(
        "webapi.get",
        true,
        Some(
            vec![
                ("token".to_string(), link.to_string()),
                ("type".to_string(), "song".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    let songs = parse_song_value(result)
        .into_iter()
        .map(|song| song_payload(song))
        .collect();

    Ok(songs)
}

/// Helper function to make request to `reco.getreco` endpoint of JioSaavn API to get song recommendations
///
/// ## Arguments
///
/// * `id` - Song id
///
/// ## Returns
///
/// * `Result<Vec<SongResponse>, reqwest::Error>` - Result of song payload
pub async fn get_song_recommendations(id: &str) -> Result<Vec<SongResponse>, reqwest::Error> {
    let result: Vec<SongRequest> = http(
        "reco.getreco",
        true,
        Some(
            vec![("pid".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    let songs = result.into_iter().map(|song| song_payload(song)).collect();

    Ok(songs)
}

/// Utility function to parse song from serde_json::Value
///
/// ## Arguments
///
/// * `serde_value` - serde_json::Value
///
/// ## Returns
///
/// * `Vec<SongRequest>` - Vector of SongRequest
fn parse_song_value(serde_value: Value) -> Vec<SongRequest> {
    match serde_value["songs"].as_array() {
        Some(songs) => songs
            .iter()
            .map(|song| from_value(song.clone()).unwrap())
            .collect(),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_song_details_by_id() -> Result<(), reqwest::Error> {
        let result = get_song_details_by_id("5WXAlMNt,9BjJPi0Y").await?;

        dbg!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_song_details_by_link() -> Result<(), reqwest::Error> {
        let result =
            get_song_details_by_link("https://www.jiosaavn.com/song/thunderclouds/RT8zcBh9eUc")
                .await?;

        dbg!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_song_recommendations() -> Result<(), reqwest::Error> {
        let result = get_song_recommendations("5WXAlMNt").await?;

        dbg!("{:?}", result);

        Ok(())
    }
}
