use super::api_service::http;

pub async fn get_top_searches() -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http("content.getTopSearches", false, None).await?;

    Ok(result)
}

pub async fn search_all(query: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "autocomplete.get",
        false,
        Some(
            vec![("query".to_string(), query.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result)
}

pub async fn search_songs(
    query: &str,
    page: u32,
    limit: u32,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "search.getResults",
        false,
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

    Ok(result)
}

pub async fn search_albums(
    query: &str,
    page: u32,
    limit: u32,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
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

    Ok(result)
}

pub async fn search_playlists(
    query: &str,
    page: u32,
    limit: u32,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "search.getPlaylistResults",
        false,
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

    Ok(result)
}

pub async fn search_artists(
    query: &str,
    page: u32,
    limit: u32,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "search.getArtistResults",
        false,
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

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_searches() {
        let result = get_top_searches().await;

        println!("{:?}", result);
    }
}
