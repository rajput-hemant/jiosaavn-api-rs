use serde_json::Value;

use super::api_service::http;
use crate::{
    config::{RECOMMEND_SONGS, SONG_BY_ID, SONG_BY_LINK},
    models::{
        misc::Union,
        response::{CResponse, Status},
        song::{RSong, RSongReco},
    },
    payloads::{song_obj_payload, songs_payload},
    utils::{formatted_payload, token_from_link, valid_langs},
};

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
/// * `Result<RSong, String>` - Song's details
pub async fn get_song_details(
    id: String,
    token: String,
    link: String,
    raw: bool,
    camel: bool,
) -> Result<RSong, String> {
    let path = if id.is_empty() {
        SONG_BY_LINK
    } else {
        SONG_BY_ID
    };

    let token = match token.is_empty() {
        true => token_from_link(link),
        false => token,
    };

    let query = vec![
        ("pids".to_string(), id),
        ("token".to_string(), token),
        ("type".to_string(), "song".to_string()),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(path, true, Some(query)).await;
    let err_msg = "❌ Song not found, please check the id, token or link".to_string();

    match response {
        Ok(obj) => {
            if obj.get("songs").is_none() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(obj))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Song details fetched successfully".to_string(),
                    Some(formatted_payload(obj, camel, &song_obj_payload)),
                )))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Err(err_msg)
        }
    }
}

/// Helper function to get song recommendations by id
///
/// ## Arguments
///
/// * `id` - Song id
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `Result<RSongReco, String>` - Song's recommendations
pub async fn get_song_recommendations(
    id: String,
    lang: String,
    raw: bool,
    camel: bool,
) -> Result<RSongReco, String> {
    let query = vec![
        ("pid".to_string(), id),
        ("language".to_string(), valid_langs(lang)),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(RECOMMEND_SONGS, true, Some(query)).await;
    let err_msg = "❌ No recommendations found, please check the id".to_string();

    match response {
        Ok(songs) => {
            if songs.is_null() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(songs))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Song Recommendations fetched successfully".to_string(),
                    Some(formatted_payload(songs, camel, &songs_payload)),
                )))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Err(err_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_song_details_by_id() {
        let result = get_song_details(
            "HLulXlir,9BjJPi0Y".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(obj) => {
                        assert_eq!(obj.songs[0].id, "HLulXlir");
                    }
                }
            }
        }
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

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(obj) => {
                        assert_eq!(obj.songs[0].id, "HLulXlir");
                    }
                }
            }
        }
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

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(obj) => {
                        assert_eq!(obj.songs[0].id, "HLulXlir");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_song_details_raw() {
        let result = get_song_details(
            "HLulXlir,9BjJPi0Y".to_string(),
            "".to_string(),
            "".to_string(),
            true,
            false,
        )
        .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(res) => {
                assert!(res["songs"].is_array());

                assert_eq!(res["songs"][0]["id"], "HLulXlir");
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_song_details_camel() {
        let result = get_song_details(
            "HLulXlir,9BjJPi0Y".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            true,
        )
        .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(res) => {
                assert_eq!(res["status"], "Success");
                assert!(res["data"].is_object());
                assert!(res["data"]["songs"].is_array());

                assert_eq!(res["data"]["songs"][0]["id"], "HLulXlir");
                assert!(res["data"]["songs"][0]["headerDesc"].is_string());
                assert!(res["data"]["songs"][0]["playCount"].is_u64());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_song_details_invalid() {
        let result = get_song_details(
            "5WXA____".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ Song not found, please check the id, token or link".to_string()
        );
    }

    #[tokio::test]
    async fn test_get_song_recommendations() {
        let result =
            get_song_recommendations("HLulXlir".to_string(), "hindi".to_string(), false, false)
                .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(songs) => {
                        assert!(!songs.is_empty());
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_song_recommendations_raw() {
        let result =
            get_song_recommendations("HLulXlir".to_string(), "hindi".to_string(), true, false)
                .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(res) => {
                assert!(res.is_array());
                assert!(res[0]["id"].is_string());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_song_recommendations_camel() {
        let result =
            get_song_recommendations("HLulXlir".to_string(), "hindi".to_string(), false, true)
                .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(res) => {
                assert_eq!(res["status"], "Success");
                assert!(res["data"].is_array());

                assert!(res["data"][0]["id"].is_string());
                assert!(res["data"][0]["headerDesc"].is_string());
                assert!(res["data"][0]["playCount"].is_u64());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_song_recommendations_invalid() {
        let result =
            get_song_recommendations("5WXAl____".to_string(), "hindi".to_string(), false, false)
                .await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ No recommendations found, please check the id".to_string()
        );
    }
}
