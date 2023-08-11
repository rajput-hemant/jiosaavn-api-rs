use reqwest::Error;

use super::api_service::http;
use crate::models::lyrics::Lyrics;

/// Helper function to make request to `lyrics.getLyrics` endpoint of JioSaavn API to get song lyrics
///
/// ## Arguments
///
/// * `song_id` - Song id
///
/// ## Returns
///
/// * `Result<Lyrics, Error>` - Result of lyrics payload
pub async fn get_lyrics(song_id: &str) -> Result<Lyrics, Error> {
    let result = http(
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
    async fn test_get_lyrics() -> Result<(), Error> {
        let result = get_lyrics("qMehA7mk").await?;

        dbg!("{:#?}", result);

        Ok(())
    }
}
