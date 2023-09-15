use super::api_service::http;
use crate::{
    config::{RECOMMEND_SONGS, SONG_BY_ID, SONG_BY_LINK},
    models::{
        misc::Union,
        response::{CResponse, Status},
        song::{RSong, RSongReco, SongRequest},
    },
    payloads::song_payload::{song_obj_payload, song_payload},
    utils::token_from_link,
};
use serde_json::from_value;

/// Helper function to get song details by id, token or link
///
/// ## Arguments
///
/// * `ids` - Comma separated song ids
/// * `link` - Song link
/// * `token` - Song token
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `RSong` - Result of song payload
pub async fn get_song_details(
    id: String,
    token: String,
    link: String,
    raw: bool,
    _: bool,
) -> RSong {
    let path = if id.is_empty() {
        SONG_BY_LINK
    } else {
        SONG_BY_ID
    };

    let query = vec![
        ("pids".to_string(), id),
        if token.is_empty() {
            ("token".to_string(), token_from_link(link))
        } else {
            ("token".to_string(), token)
        },
        ("type".to_string(), "song".to_string()),
    ]
    .into_iter()
    .collect();

    let response = http(path, true, Some(query)).await;

    match response {
        Ok(obj) => {
            if raw {
                Union::A(obj)
            } else {
                // TODO: Add camel case conversion

                Union::B(CResponse::new(
                    Status::Success,
                    "✅ Song details fetched successfully",
                    Some(song_obj_payload(from_value(obj).unwrap())),
                ))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Union::B(CResponse::new(
                Status::Failed,
                "❌ Song not found, please check the id, token or link",
                None,
            ))
        }
    }
}

/// Helper function to get song recommendations by id
///
/// ## Arguments
///
/// * `id` - Song id
///
/// ## Returns
///
/// * `Result<Vec<SongResponse>, reqwest::Error>` - Result of song payload
pub async fn get_song_recommendations(id: String, lang: String, raw: bool, _: bool) -> RSongReco {
    let response = http(
        RECOMMEND_SONGS,
        true,
        Some(
            vec![("pid".to_string(), id), ("language".to_string(), lang)]
                .into_iter()
                .collect(),
        ),
    )
    .await;

    match response {
        Ok(songs) => {
            if raw {
                Union::A(songs)
            } else {
                // TODO: Add camel case conversion

                Union::B(CResponse::new(
                    Status::Success,
                    "✅ Song Recommendations fetched successfully",
                    Some(
                        from_value::<Vec<SongRequest>>(songs)
                            .unwrap()
                            .into_iter()
                            .map(song_payload)
                            .collect(),
                    ),
                ))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Union::B(CResponse::new(
                Status::Failed,
                "❌ No recommendations found, please check the id",
                None,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_song_details_by_id() {
        let result = get_song_details(
            "5WXAlMNt,9BjJPi0Y".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_details_by_id_raw() {
        let result = get_song_details(
            "5WXAlMNt,9BjJPi0Y".to_string(),
            "".to_string(),
            "".to_string(),
            true,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_details_by_token() {
        let result = get_song_details(
            "".to_string(),
            "OCQeXSxcXkE".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_details_by_token_raw() {
        let result = get_song_details(
            "".to_string(),
            "OCQeXSxcXkE".to_string(),
            "".to_string(),
            true,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_details_by_link() {
        let result = get_song_details(
            "".to_string(),
            "".to_string(),
            "https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE".to_string(),
            false,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_details_by_link_raw() {
        let result = get_song_details(
            "".to_string(),
            "".to_string(),
            "https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE".to_string(),
            true,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_recommendations() {
        let result =
            get_song_recommendations("5WXAlMNt".to_string(), "hindi".to_string(), false, false)
                .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_song_recommendations_raw() {
        let result =
            get_song_recommendations("5WXAlMNt".to_string(), "hindi".to_string(), true, false)
                .await;

        dbg!("{:?}", result);
    }
}
