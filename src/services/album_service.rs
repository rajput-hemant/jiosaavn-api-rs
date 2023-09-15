use serde_json::from_value;

use super::api_service::http;
use crate::{
    config::{ALBUM_BY_ID, ALBUM_BY_LINK, ALBUM_BY_SAME_YEAR, RECOMMEND_ALBUMS},
    models::{
        album::{AlbumRequest, AlbumResponse, RAlbum, RAlbumVec},
        misc::Union,
        response::{CResponse, Status},
    },
    payloads::album_payload,
    utils::token_from_link,
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
/// * `RAlbum` - Result of album payload
pub async fn get_album_details(
    id: String,
    token: String,
    link: String,
    raw: bool,
    _: bool,
) -> RAlbum {
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

    let response = http(path, true, Some(query)).await;

    let error_payload = Union::B(CResponse::new(
        Status::Failed,
        "❌ No album found, please check the id or link",
        None,
    ));

    match response {
        Ok(album) => {
            if raw {
                Union::A(album)
            } else {
                // TODO: Add camel case conversion

                let payload = album_payload(from_value(album).unwrap());
                if payload.id.is_empty() {
                    error_payload
                } else {
                    Union::B(CResponse::new(
                        Status::Success,
                        "✅ Album details fetched successfully",
                        Some(payload),
                    ))
                }
            }
        }

        Err(e) => {
            println!("Error: {e}");

            error_payload
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
/// * `RAlbumVec` - Album recommendations
pub async fn get_album_recommendations(id: String, lang: String, raw: bool, _: bool) -> RAlbumVec {
    let query = vec![("albumid".to_string(), id), ("language".to_string(), lang)]
        .into_iter()
        .collect();

    let response = http(RECOMMEND_ALBUMS, true, Some(query)).await;

    let error_payload = Union::B(CResponse::new(
        Status::Failed,
        "❌ No recommendations found, please check the album id",
        None,
    ));

    match response {
        Ok(albums) => {
            if raw {
                Union::A(albums)
            } else {
                // TODO: Add camel case conversion

                let payload: Vec<AlbumResponse> = from_value::<Vec<AlbumRequest>>(albums)
                    .unwrap()
                    .into_iter()
                    .map(album_payload)
                    .collect();

                if payload.is_empty() {
                    error_payload
                } else {
                    Union::B(CResponse::new(
                        Status::Success,
                        "✅ Album Recommendations fetched successfully",
                        Some(payload),
                    ))
                }
            }
        }

        Err(e) => {
            println!("Error: {e}");

            error_payload
        }
    }
}

/// Helper function to get albums from same year
///
/// ## Arguments
///
/// * `year` - Album year
/// * `lang` - Album language
///
/// ## Returns
///
/// * `RAlbumVec` - Albums from same year
pub async fn get_album_from_same_year(year: String, lang: String, raw: bool, _: bool) -> RAlbumVec {
    let query = vec![
        ("album_year".to_string(), year),
        ("album_lang".to_string(), lang),
    ]
    .into_iter()
    .collect();

    let response = http(ALBUM_BY_SAME_YEAR, true, Some(query)).await;
    let error_payload = Union::B(CResponse::new(
        Status::Failed,
        "❌ No albums found, please check the year",
        None,
    ));

    match response {
        Ok(albums) => {
            if raw {
                Union::A(albums)
            } else {
                // TODO: Add camel case conversion

                let payload: Vec<AlbumResponse> = from_value::<Vec<AlbumRequest>>(albums)
                    .unwrap()
                    .into_iter()
                    .map(album_payload)
                    .collect();

                if payload.is_empty() {
                    error_payload
                } else {
                    Union::B(CResponse::new(
                        Status::Success,
                        "✅ Album fetched successfully",
                        Some(payload),
                    ))
                }
            }
        }

        Err(e) => {
            println!("Error: {e}");

            error_payload
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_album_details_by_id() {
        let result = get_album_details(
            "1142502".to_string(),
            "".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_details_by_id_raw() {
        let result = get_album_details(
            "1142502".to_string(),
            "".to_string(),
            "".to_string(),
            true,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_details_by_token() {
        let result = get_album_details(
            "".to_string(),
            "xe6Gx7Sg12U".to_string(),
            "".to_string(),
            false,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_details_by_token_raw() {
        let result = get_album_details(
            "".to_string(),
            "xe6Gx7Sg12U".to_string(),
            "".to_string(),
            true,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_details_by_link() {
        let result = get_album_details(
            "".to_string(),
            "".to_string(),
            "https://www.jiosaavn.com/album/night-visions/xe6Gx7Sg12U".to_string(),
            false,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_details_by_link_raw() {
        let result = get_album_details(
            "".to_string(),
            "".to_string(),
            "https://www.jiosaavn.com/album/night-visions/xe6Gx7Sg12U".to_string(),
            true,
            false,
        )
        .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_recommendations() {
        let result =
            get_album_recommendations("1142502".to_string(), "hindi".to_string(), false, false)
                .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_recommendations_raw() {
        let result =
            get_album_recommendations("1142502".to_string(), "hindi".to_string(), true, false)
                .await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_from_same_year() {
        let result =
            get_album_from_same_year("2023".to_string(), "hindi".to_string(), false, false).await;

        dbg!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_album_from_same_year_raw() {
        let result =
            get_album_from_same_year("2023".to_string(), "hindi".to_string(), true, false).await;

        dbg!("{:?}", result);
    }
}
