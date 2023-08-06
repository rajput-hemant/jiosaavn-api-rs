use crate::{
    models::album::{AlbumRequest, AlbumResponse},
    payloads::album_paylaod::album_payload,
};

use super::api_service::http;

/// Helper function to make request to `content.getAlbumDetails` endpoint of JioSaavn API to get album details
///
/// ## Arguments
///
/// * `id` - Album id
///
/// ## Returns
///
/// * `Result<AlbumResponse, reqwest::Error>` - Result of album payload
pub async fn get_album_details_by_id(id: &str) -> Result<AlbumResponse, reqwest::Error> {
    let result = http(
        "content.getAlbumDetails",
        true,
        Some(
            vec![("albumid".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(album_payload(result))
}

/// Helper function to make request to `webapi.get` endpoint of JioSaavn API to get album details
///
/// ## Arguments
///
/// * `link` - Album link
///
/// ## Returns
///
/// * `Result<AlbumResponse, reqwest::Error>` - Result of album payload
pub async fn get_album_details_by_link(link: &str) -> Result<AlbumResponse, reqwest::Error> {
    let result = http(
        "webapi.get",
        true,
        Some(
            vec![
                ("token".to_string(), link.to_string()),
                ("type".to_string(), "album".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(album_payload(result))
}

/// Helper function to make request to `reco.getAlbumReco` endpoint of JioSaavn API to get album recommendations
///
/// ## Arguments
///
/// * `id` - Album id
///
/// ## Returns
///
/// * `Result<Vec<AlbumResponse>, reqwest::Error>` - Result of album payload
pub async fn get_album_recommendations(id: &str) -> Result<Vec<AlbumResponse>, reqwest::Error> {
    let result: Vec<AlbumRequest> = http(
        "reco.getAlbumReco",
        true,
        Some(
            vec![("albumid".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result
        .into_iter()
        .map(|album| album_payload(album))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_album_details_by_id() -> Result<(), reqwest::Error> {
        let result = get_album_details_by_id("1142502").await?;

        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_album_details_by_link() -> Result<(), reqwest::Error> {
        let result =
            get_album_details_by_link("https://www.jiosaavn.com/album/night-visions/xe6Gx7Sg12U_")
                .await?;

        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_album_recommendations() -> Result<(), reqwest::Error> {
        let result = get_album_recommendations("1142502").await?;

        println!("{:?}", result);

        Ok(())
    }
}
