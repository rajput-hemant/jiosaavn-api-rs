use super::api_service::http;

pub async fn get_artist_details_by_id(id: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "artist.getArtistPageDetails",
        true,
        Some(
            vec![("artistId".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result)
}

pub async fn get_artist_details_by_link(link: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "webapi.get",
        true,
        Some(
            vec![
                ("token".to_string(), link.to_string()),
                ("type".to_string(), "artist".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(result)
}

pub async fn get_artist_songs(
    artist_id: &str,
    page: u64,
    category: &str,
    sort: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "artist.getArtistMoreSong",
        false,
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

    Ok(result)
}

pub async fn get_artist_albums(
    artist_id: &str,
    page: u64,
    category: &str,
    sort: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
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

    Ok(result)
}

pub async fn get_artist_top_songs(
    artist_id: &str,
    song_id: &str,
    language: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "search.artistOtherTopSongs",
        false,
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

    Ok(result)
}
