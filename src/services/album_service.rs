use serde_json::Value;

use super::api_service::http;
use crate::{
    config::{ALBUM_BY_ID, ALBUM_BY_LINK, ALBUM_BY_SAME_YEAR, RECOMMEND_ALBUMS},
    models::{
        album::{RAlbum, RAlbumVec},
        misc::Union,
        response::{CResponse, Status},
    },
    payloads::{album_payload, albums_payload},
    utils::{formatted_payload, token_from_link},
};

/// Helper function to get album details by id, token or link
///
/// ## Arguments
///
/// * `ids` - Album id
/// * `link` - Album link
/// * `token` - Album token
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `Result<RAlbum, String>` - Album details
pub async fn get_album_details(
    id: String,
    token: String,
    link: String,
    raw: bool,
    camel: bool,
) -> Result<RAlbum, String> {
    let path = if id.is_empty() {
        ALBUM_BY_LINK
    } else {
        ALBUM_BY_ID
    };

    let token = match token.is_empty() {
        true => token_from_link(link),
        false => token,
    };

    let query = vec![
        ("albumid".to_string(), id),
        ("token".to_string(), token),
        ("type".to_string(), "album".to_string()),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(path, true, Some(query)).await;
    let err_msg = "❌ No album found, please check the id or link".to_string();

    match response {
        Ok(album) => {
            if album.get("id").is_none() || album["id"].as_str().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(album))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Album details fetched successfully".to_string(),
                    Some(formatted_payload(album, camel, &album_payload)),
                )))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Err(err_msg)
        }
    }
}

/// Helper function to get album recommendations by id
///
/// ## Arguments
///
/// * `id` - Album id
/// * `lang` - Album language
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `Result<RAlbumVec, String>` - Album recommendations
pub async fn get_album_recommendations(
    id: String,
    lang: String,
    raw: bool,
    camel: bool,
) -> Result<RAlbumVec, String> {
    let query = vec![("albumid".to_string(), id), ("language".to_string(), lang)]
        .into_iter()
        .collect();

    let response = http::<Value>(RECOMMEND_ALBUMS, true, Some(query)).await;
    let err_msg = "❌ No recommendations found, please check the album id".to_string();

    match response {
        Ok(albums) => {
            if albums.as_array().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(albums))
            } else {
                let payload = formatted_payload(albums, camel, &albums_payload);

                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Album Recommendations fetched successfully".to_string(),
                    Some(payload),
                )))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Err(err_msg)
        }
    }
}

/// Helper function to get albums from same year
///
/// ## Arguments
///
/// * `year` - Album year
/// * `lang` - Album language
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `Result<RAlbumVec, String>` - Albums from same year
pub async fn get_album_from_same_year(
    year: String,
    lang: String,
    raw: bool,
    camel: bool,
) -> Result<RAlbumVec, String> {
    let query = vec![
        ("album_year".to_string(), year),
        ("album_lang".to_string(), lang),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(ALBUM_BY_SAME_YEAR, true, Some(query)).await;
    let err_msg = "❌ No albums found, please check the year".to_string();

    match response {
        Ok(albums) => {
            if albums.as_array().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(albums))
            } else {
                let payload = formatted_payload(albums, camel, &albums_payload);

                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Albums fetched successfully".to_string(),
                    Some(payload),
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
    async fn test_get_album_details_by_id() {
        let result = get_album_details(
            "25500145".to_string(),
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
                    Union::B(album) => {
                        assert_eq!(album.id, "25500145");
                        assert_eq!(album.songs.unwrap()[0].id, "HLulXlir");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_album_details_by_token() {
        let result = get_album_details(
            "".to_string(),
            "cy9LsEw-pn0_".to_string(),
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
                    Union::B(album) => {
                        assert_eq!(album.id, "25500145");
                        assert_eq!(album.songs.unwrap()[0].id, "HLulXlir");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_album_details_by_link() {
        let result = get_album_details(
            "".to_string(),
            "".to_string(),
            "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_".to_string(),
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
                    Union::B(album) => {
                        assert_eq!(album.id, "25500145");
                        assert_eq!(album.songs.unwrap()[0].id, "HLulXlir");
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_album_details_raw() {
        let result = get_album_details(
            "25500145".to_string(),
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

                assert_eq!(res["id"], "25500145");
                assert!(res["list"].is_array());
                assert_eq!(res["list"][0]["id"], "HLulXlir");
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_album_details_camel() {
        let result = get_album_details(
            "25500145,9BjJPi0Y".to_string(),
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
                assert_eq!(res["data"]["id"], "25500145");
                assert!(res["data"]["songs"].is_array());

                assert_eq!(res["data"]["songs"][0]["id"], "HLulXlir");
                assert!(res["data"]["songs"][0]["headerDesc"].is_string());
                assert!(res["data"]["songs"][0]["playCount"].is_u64());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_album_details_invalid() {
        let result = get_album_details(
            "HLul____".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ No album found, please check the id or link".to_string()
        );
    }

    #[tokio::test]
    async fn test_get_album_recommendations() {
        let result =
            get_album_recommendations("25500145".to_string(), "hindi".to_string(), false, false)
                .await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(album) => {
                        assert!(!album.is_empty());
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_album_recommendations_raw() {
        let result =
            get_album_recommendations("25500145".to_string(), "hindi".to_string(), true, false)
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
    async fn test_get_album_recommendations_camel() {
        let result =
            get_album_recommendations("25500145".to_string(), "hindi".to_string(), false, true)
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
    async fn test_get_album_recommendations_invalid() {
        let result =
            get_album_recommendations("2550____".to_string(), "hindi".to_string(), false, false)
                .await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ No recommendations found, please check the album id".to_string()
        );
    }

    #[tokio::test]
    async fn test_get_album_from_same_year() {
        let result =
            get_album_from_same_year("2023".to_string(), "hindi".to_string(), false, false).await;

        assert!(result.is_ok());

        match result.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(album) => {
                        assert!(!album.is_empty());
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_album_from_same_year_raw() {
        let result =
            get_album_from_same_year("2023".to_string(), "hindi".to_string(), true, false).await;

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
    async fn test_get_album_from_same_year_camel() {
        let result =
            get_album_from_same_year("2023".to_string(), "hindi".to_string(), false, true).await;

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
    async fn test_get_album_from_same_year_invalid() {
        let result =
            get_album_from_same_year("1600".to_string(), "hindi".to_string(), false, false).await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "❌ No albums found, please check the year".to_string()
        );
    }
}
