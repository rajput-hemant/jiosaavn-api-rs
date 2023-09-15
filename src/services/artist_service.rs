use crate::{
    models::{
        artist::{ArtistAlbumsResponse, ArtistResponse, ArtistSongsResponse},
        song::{SongRequest, SongResponse},
    },
    payloads::{
        artist_payload::{artist_albums_payload, artist_payload, artist_songs_payload},
        song_payload,
    },
    utils::token_from_link,
};

use super::api_service::http;

/// Helper function to make request to `artist.getArtistPageDetails` endpoint of JioSaavn API to get artist details
///
/// ## Arguments
///
/// * `id` - Artist id
///
/// ## Returns
///
/// * `Result<ArtistResponse, reqwest::Error>` - Result of artist payload
pub async fn get_artist_details_by_id(id: &str) -> Result<ArtistResponse, reqwest::Error> {
    let result = http(
        "artist.getArtistPageDetails",
        true,
        Some(
            vec![("artistId".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(artist_payload(result))
}

/// Helper function to make request to `webapi.get` endpoint of JioSaavn API to get artist details
///
/// ## Arguments
///
/// * `link` - Artist link
///
/// ## Returns
///
/// * `Result<ArtistResponse, reqwest::Error>` - Result of artist payload
pub async fn get_artist_details_by_link(link: &str) -> Result<ArtistResponse, reqwest::Error> {
    let result = http(
        "webapi.get",
        true,
        Some(
            vec![
                ("token".to_string(), token_from_link(link.to_string())),
                ("type".to_string(), "artist".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(artist_payload(result))
}

/// Helper function to make request to `artist.getArtistMoreSong` endpoint of JioSaavn API to get artist songs
///
/// ## Arguments
///
/// * `artist_id` - Artist id
/// * `page` - Page number
/// * `category` - Category of songs `[latest, alphabetical]`
/// * `sort` - Sort order `[asc, desc]`
///
/// ## Returns
///
/// * `Result<ArtistSongsResponse, reqwest::Error>` - Result of artist songs payload
pub async fn get_artist_songs(
    artist_id: &str,
    page: u64,
    category: &str,
    sort: &str,
) -> Result<ArtistSongsResponse, reqwest::Error> {
    let result = http(
        "artist.getArtistMoreSong",
        true,
        Some(
            vec![
                ("artistId".to_string(), artist_id.to_string()),
                ("page".to_string(), (page - 1).to_string()),
                ("category".to_string(), category.to_string()),
                ("sort_order".to_string(), sort.to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(artist_songs_payload(result))
}

/// Helper function to make request to `artist.getArtistMoreAlbum` endpoint of JioSaavn API to get artist albums
///
/// ## Arguments
///
/// * `artist_id` - Artist id
/// * `page` - Page number
/// * `category` - Category of albums `[latest, alphabetical]`
/// * `sort` - Sort order `[asc, desc]`
///
/// ## Returns
///
/// * `Result<ArtistAlbumsResponse, reqwest::Error>` - Result of artist albums payload
pub async fn get_artist_albums(
    artist_id: &str,
    page: u64,
    category: &str,
    sort: &str,
) -> Result<ArtistAlbumsResponse, reqwest::Error> {
    let result = http(
        "artist.getArtistMoreAlbum",
        true,
        Some(
            vec![
                ("artistId".to_string(), artist_id.to_string()),
                ("page".to_string(), (page - 1).to_string()),
                ("category".to_string(), category.to_string()),
                ("sort_order".to_string(), sort.to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(artist_albums_payload(result))
}

/// Helper function to make request to `search.artistOtherTopSongs` endpoint of JioSaavn API to get artist recommended songs
///
/// ## Arguments
///
/// * `artist_id` - Artist id
/// * `song_id` - Song id
/// * `languages` - Comma separated languages
/// * Available languages: `hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,
///  `gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`,
///  `rajasthani`, `odia`, `assamese`
///
/// ## Returns
///
/// * `Result<Vec<SongResponse>, reqwest::Error>` - Result of artist recommended songs payload
pub async fn get_artist_top_songs(
    artist_id: &str,
    song_id: &str,
    language: &str,
) -> Result<Vec<SongResponse>, reqwest::Error> {
    let result: Vec<SongRequest> = http(
        "search.artistOtherTopSongs",
        true,
        Some(
            vec![
                ("artist_ids".to_string(), artist_id.to_string()),
                ("song_id".to_string(), song_id.to_string()),
                ("language".to_string(), language.to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(result.into_iter().map(song_payload).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_artist_details_by_id() -> Result<(), reqwest::Error> {
        let result = get_artist_details_by_id("568707").await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_artist_details_by_link() -> Result<(), reqwest::Error> {
        let result =
            get_artist_details_by_link("https://www.jiosaavn.com/artist/sia-/C4hxFiXrHws_").await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_artist_songs() -> Result<(), reqwest::Error> {
        let result = get_artist_songs("568707", 1, "latest", "asc").await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_artist_albums() -> Result<(), reqwest::Error> {
        let result = get_artist_albums("568707", 1, "latest", "asc").await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_artist_top_songs() -> Result<(), reqwest::Error> {
        let result = get_artist_top_songs("459320", "_rJmbKSP", "english").await?;

        dbg!(result);

        Ok(())
    }
}
