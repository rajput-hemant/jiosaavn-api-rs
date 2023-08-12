use crate::{models::playlist::PlaylistResponse, payloads::playlist_payload};

use super::api_service::http;

/// Helper function to make request to `playlist.getDetails` endpoint of JioSaavn API to get playlist details
///
/// ## Arguments
///
/// * `id` - Playlist id
///
/// ## Returns
///
/// * `Result<PlaylistResponse, reqwest::Error>` - Result of playlist payload
pub async fn get_playlist_details_by_id(id: &str) -> Result<PlaylistResponse, reqwest::Error> {
    let result = http(
        "playlist.getDetails",
        true,
        Some(
            vec![("listid".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(playlist_payload(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_playlist_details_by_id() -> Result<(), reqwest::Error> {
        let result = get_playlist_details_by_id("159144718").await?;

        dbg!("{:?}", result);

        Ok(())
    }
}
