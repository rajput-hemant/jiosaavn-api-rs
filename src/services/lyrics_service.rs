use super::api_service::http;

/// Helper function to make request to `lyrics.getLyrics` endpoint of JioSaavn API to get song lyrics
///
/// ## Arguments
///
/// * `song_id` - Song id
///
/// ## Returns
///
/// * `Result<serde_json::Value, reqwest::Error>` - Result of lyrics payload
pub async fn get_lyrics(song_id: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "lyrics.getLyrics",
        true,
        Some(
            vec![("lyrics_id".to_string(), song_id.to_string())]
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
    async fn test_get_lyrics() {
        let result = get_lyrics("mPTrDSun").await.unwrap();

        println!("{:#?}", result)
    }
}
