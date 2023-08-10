use reqwest::Error;

use crate::{
    models::{
        album::AlbumResponse,
        search::{
            AllSearchResponse, SearchArtistResponse, SearchPlaylistResponse, TSearchResponse,
            TopSearchesResponse,
        },
        song::SongResponse,
    },
    payloads::{
        album_search_payload, all_search_payload, artist_search_payload, playlist_search_payload,
        song_search_payload, top_searches_payload,
    },
};

use super::api_service::http;

/// Helper function to make request to `content.getTopSearches` endpoint of JioSaavn API to get top searches
///
/// ## Returns
///
/// * `Result<Vec<TopSearchesResponse>, Error>` - Result of top searches payload
pub async fn get_top_searches() -> Result<Vec<TopSearchesResponse>, Error> {
    let result = http("content.getTopSearches", true, None).await?;

    Ok(top_searches_payload(result))
}

/// Helper function to make request to `autocomplete.get` endpoint of JioSaavn API to get all search results
///
/// ## Arguments
///
/// * `query` - Search query
///
/// ## Returns
///
/// * `Result<AllSearchResponse, Error>` - Result of all search payload
pub async fn search_all(query: &str) -> Result<AllSearchResponse, Error> {
    let result = http(
        "autocomplete.get",
        true,
        Some(
            vec![("query".to_string(), query.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(all_search_payload(result))
}

/// Helper function to make request to `search.getResults` endpoint of JioSaavn API to get song search results
///
/// ## Arguments
///
/// * `query` - Search query
/// * `page` - Page number
/// * `limit` - Number of results per page
///
/// ## Returns
///
/// * `Result<TSearchResponse<SongResponse>, Error>` - Result of song search payload
pub async fn search_songs(
    query: &str,
    page: u64,
    limit: u64,
) -> Result<TSearchResponse<SongResponse>, Error> {
    let result = http(
        "search.getResults",
        true,
        Some(
            vec![
                ("q".to_string(), query.to_string()),
                ("p".to_string(), page.to_string()),
                ("n".to_string(), limit.to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(song_search_payload(result))
}

/// Helper function to make request to `search.getAlbumResults` endpoint of JioSaavn API to get album search results
///
/// ## Arguments
///
/// * `query` - Search query
/// * `page` - Page number
/// * `limit` - Number of results per page
///
/// ## Returns
///
/// * `Result<TSearchResponse<AlbumResponse>, Error>` - Result of album search payload
pub async fn search_albums(
    query: &str,
    page: u64,
    limit: u64,
) -> Result<TSearchResponse<AlbumResponse>, Error> {
    let result = http(
        "search.getAlbumResults",
        true,
        Some(
            vec![
                ("q".to_string(), query.to_string()),
                ("p".to_string(), page.to_string()),
                ("n".to_string(), limit.to_string()),
                ("type".to_string(), "album".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(album_search_payload(result))
}

/// Helper function to make request to `search.getPlaylistResults` endpoint of JioSaavn API to get playlist search results
///
/// ## Arguments
///
/// * `query` - Search query
/// * `page` - Page number
/// * `limit` - Number of results per page
///
/// ## Returns
///
/// * `Result<TSearchResponse<SearchPlaylistResponse>, Error>` - Result of playlist search payload
pub async fn search_playlists(
    query: &str,
    page: u64,
    limit: u64,
) -> Result<TSearchResponse<SearchPlaylistResponse>, Error> {
    let result = http(
        "search.getPlaylistResults",
        true,
        Some(
            vec![
                ("q".to_string(), query.to_string()),
                ("p".to_string(), page.to_string()),
                ("n".to_string(), limit.to_string()),
                ("type".to_string(), "playlist".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(playlist_search_payload(result))
}

/// Helper function to make request to `search.getArtistResults` endpoint of JioSaavn API to get artist search results
///
/// ## Arguments
///
/// * `query` - Search query
/// * `page` - Page number
/// * `limit` - Number of results per page
///
/// ## Returns
///
/// * `Result<TSearchResponse<SearchArtistResponse>, Error>` - Result of artist search payload
pub async fn search_artists(
    query: &str,
    page: u64,
    limit: u64,
) -> Result<TSearchResponse<SearchArtistResponse>, Error> {
    let result = http(
        "search.getArtistResults",
        true,
        Some(
            vec![
                ("q".to_string(), query.to_string()),
                ("p".to_string(), page.to_string()),
                ("n".to_string(), limit.to_string()),
                ("type".to_string(), "artist".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(artist_search_payload(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_searches() -> Result<(), Error> {
        let result = get_top_searches().await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_all() -> Result<(), Error> {
        let result = search_all("punjab se bollywood").await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_songs() -> Result<(), Error> {
        let result = search_songs("ram siya ram", 1, 10).await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_albums() -> Result<(), Error> {
        let result = search_albums("welcome", 1, 10).await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_playlists() -> Result<(), Error> {
        let result = search_playlists("punjab se bollywood", 1, 10).await?;

        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_artists() -> Result<(), Error> {
        let result = search_artists("arijit singh", 1, 10).await?;

        dbg!(result);

        Ok(())
    }
}
