use serde_json::Value;

use crate::{
    config::{PLAYLIST_BY_ID, PLAYLIST_BY_LINK, RECOMMEND_PLAYLISTS},
    models::{
        misc::Union,
        playlist::{RPlaylist, RPlaylistVec},
        response::{CResponse, Status},
    },
    payloads::playlist_payload::{playlist_payload, playlist_recommend_payload},
    utils::{formatted_payload, token_from_link},
};

use super::api_service::http;

/// Helper function to get playlist details by id, token or link
///
/// ## Arguments
///
/// * `id` - Playlist id
///
/// ## Returns
///
/// * `Result<RPlaylist, String>` - Playlist's details
pub async fn get_playlist_details(
    id: String,
    token: String,
    link: String,
    raw: bool,
    camel: bool,
) -> Result<RPlaylist, String> {
    let path = if id.is_empty() {
        PLAYLIST_BY_LINK
    } else {
        PLAYLIST_BY_ID
    };

    let token = match token.is_empty() {
        true => token_from_link(link),
        false => token,
    };

    let query = vec![
        ("listid".to_string(), id),
        ("token".to_string(), token),
        ("type".to_string(), "playlist".to_string()),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(path, true, Some(query)).await;
    let err_msg = "❌ Playlist not found, please check the id, token or link".to_string();

    match response {
        Ok(playlist) => {
            if playlist.get("id").is_none() || playlist["id"].as_str().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(playlist))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Playlist details fetched successfully".to_string(),
                    Some(formatted_payload(playlist, camel, &playlist_payload)),
                )))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Err(err_msg)
        }
    }
}

/// Helper function to get playlist recommendations by id
/// 
/// ## Arguments
/// 
/// * `id` - Playlist id
/// * `lang` - Language of the recommendations
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
/// 
/// ## Returns
/// 
/// * `Result<RPlaylistVec, String>` - List of recommended playlists
pub async fn get_playlist_recommendations(
    id: String,
    lang: String,
    raw: bool,
    camel: bool,
) -> Result<RPlaylistVec, String> {
    let query = vec![("listid".to_string(), id), ("language".to_string(), lang)]
        .into_iter()
        .collect();

    let response = http::<Value>(RECOMMEND_PLAYLISTS, true, Some(query)).await;
    let err_msg = "❌ No recommendations found, please check the playlist id".to_string();

    match response {
        Ok(playlists) => {
            if playlists.as_array().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(playlists))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Playlists Recommendations fetched successfully".to_string(),
                    Some(formatted_payload(
                        playlists,
                        camel,
                        &playlist_recommend_payload,
                    )),
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
    async fn test_get_playlist_details_by_id() {
        let result = get_playlist_details(
            "159144718".to_string(),
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
                    Union::B(playlist) => {
                        assert_eq!(playlist.id, "159144718");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_playlist_details_by_token() {
        let result = get_playlist_details(
            "".to_string(),
            "Er9SBlcxDtZFo9wdEAzFBA__".to_string(),
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
                    Union::B(playlist) => {
                        assert_eq!(playlist.id, "159144718");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_playlist_details_by_link() {
        let result = get_playlist_details(
            "".to_string(),
            "".to_string(),
            "https://www.jiosaavn.com/featured/feel-good-pop/Er9SBlcxDtZFo9wdEAzFBA__".to_string(),
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
                    Union::B(playlist) => {
                        assert_eq!(playlist.id, "159144718");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_playlist_details_raw() {
        let result = get_playlist_details(
            "159144718".to_string(),
            "".to_string(),
            "".to_string(),
            true,
            false,
        )
        .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(res) => {
                assert!(res.is_object());

                assert_eq!(res["id"], "159144718");
                assert!(res["list"].is_array());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_playlist_details_camel() {
        let result = get_playlist_details(
            "159144718".to_string(),
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
                assert_eq!(res["data"]["id"], "159144718");
                assert!(res["data"]["songs"].is_array());

                assert!(res["data"]["songs"][0]["headerDesc"].is_string());
                assert!(res["data"]["songs"][0]["playCount"].is_u64());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_playlist_details_invalid() {
        let result = get_playlist_details(
            "15914____".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ Playlist not found, please check the id, token or link".to_string()
        );
    }

    #[tokio::test]
    async fn test_get_playlist_recommendations() {
        let result = get_playlist_recommendations(
            "159144718".to_string(),
            "hindi".to_string(),
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
                    Union::B(playlist) => {
                        assert!(!playlist.is_empty());
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_playlist_recommendations_raw() {
        let result =
            get_playlist_recommendations("159144718".to_string(), "hindi".to_string(), true, false)
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
    async fn test_get_playlist_recommendations_camel() {
        let result =
            get_playlist_recommendations("159144718".to_string(), "hindi".to_string(), false, true)
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
    async fn test_get_playlist_recommendations_invalid() {
        let result =
            get_playlist_recommendations("2550____".to_string(), "hindi".to_string(), false, false)
                .await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ No recommendations found, please check the playlist id".to_string()
        );
    }
}
