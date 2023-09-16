use serde_json::from_value;

use crate::{
    config::{PLAYLIST_BY_ID, PLAYLIST_BY_LINK, RECOMMEND_PLAYLISTS},
    models::{
        misc::Union,
        playlist::{PlaylistRecRequest, PlaylistRecResponse, RPlaylist, RPlaylistRec},
        response::{CResponse, Status},
    },
    payloads::playlist_payload::{playlist_payload, playlist_recommend_payload},
    utils::token_from_link,
};

use super::api_service::http;

/// Helper function to make request to `playlist.getDetails` endpoint of JioSaavn API to get playlist details
///
/// ## Arguments
///
/// * `id` - Playlist id
///
/// ## Returns
///
/// * `Result<PlaylistResponse, reqwest::Error>` - Result of playlist payload
// pub async fn get_playlist_details_by_id(id: &str) -> Result<PlaylistResponse, reqwest::Error> {
//     let result = http(
//         "playlist.getDetails",
//         true,
//         Some(
//             vec![("listid".to_string(), id.to_string())]
//                 .into_iter()
//                 .collect(),
//         ),
//     )
//     .await?;

//     Ok(playlist_payload(result))
// }
pub async fn get_playlist_details(
    id: String,
    token: String,
    link: String,
    raw: bool,
    _: bool,
) -> RPlaylist {
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

    let response = http(path, true, Some(query)).await;

    let error_payload = Union::B(CResponse::new(
        Status::Failed,
        "❌ Playlist not found, please check the id, token or link",
        None,
    ));

    match response {
        Ok(playlist) => {
            if raw {
                Union::A(playlist)
            } else {
                // TODO: Add camel case conversion

                let payload = playlist_payload(from_value(playlist).unwrap());
                if payload.id.is_empty() {
                    error_payload
                } else {
                    Union::B(CResponse::new(
                        Status::Success,
                        "✅ Playlist details fetched successfully",
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

pub async fn get_playlist_recommendations(
    id: String,
    lang: String,
    raw: bool,
    _: bool,
) -> RPlaylistRec {
    let query = vec![("listid".to_string(), id), ("language".to_string(), lang)]
        .into_iter()
        .collect();

    let response = http(RECOMMEND_PLAYLISTS, true, Some(query)).await;

    let error_payload = Union::B(CResponse::new(
        Status::Failed,
        "❌ No recommendations found, please check the playlist id",
        None,
    ));

    match response {
        Ok(playlists) => {
            if raw {
                Union::A(playlists)
            } else {
                // TODO: Add camel case conversion

                let payload: Vec<PlaylistRecResponse> =
                    from_value::<Vec<PlaylistRecRequest>>(playlists)
                        .unwrap()
                        .into_iter()
                        .map(playlist_recommend_payload)
                        .collect();

                if payload.is_empty() {
                    error_payload
                } else {
                    Union::B(CResponse::new(
                        Status::Success,
                        "✅ Playlists Recommendations fetched successfully",
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_get_playlist_details_by_id() -> Result<(), reqwest::Error> {
//         let result = get_playlist_details_by_id("159144718").await?;

//         dbg!("{:?}", result);

//         Ok(())
//     }
// }
