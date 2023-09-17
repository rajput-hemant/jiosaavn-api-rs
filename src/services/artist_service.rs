use serde_json::Value;

use crate::{
    config::{ARTISTS_ALBUMS, ARTISTS_SONGS, ARTISTS_TOP_SONGS, ARTIST_BY_ID, ARTIST_BY_LINK},
    models::{
        artist::{RArtist, RArtistSongsOrAlbums, RArtistTopSongs},
        misc::Union,
        response::{CResponse, Status},
    },
    payloads::{
        artist_payload::artist_payload, artist_songs_or_albums_payload, song_payload::songs_payload,
    },
    utils::{formatted_payload, token_from_link},
};

use super::api_service::http;

#[allow(clippy::too_many_arguments)]
pub async fn get_artists_details(
    id: String,
    token: String,
    link: String,
    page: String,
    n_song: String,
    n_album: String,
    raw: bool,
    camel: bool,
) -> Result<RArtist, String> {
    let path = if id.is_empty() {
        ARTIST_BY_LINK
    } else {
        ARTIST_BY_ID
    };

    let token = match token.is_empty() {
        true => token_from_link(link),
        false => token,
    };

    let query = vec![
        ("artistId".to_string(), id),
        ("token".to_string(), token),
        ("type".to_string(), "artist".to_string()),
        ("p".to_string(), page),
        ("n_song".to_string(), n_song),
        ("n_album".to_string(), n_album),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(path, true, Some(query)).await;
    let err_msg = "❌ No artist found, please check the id, link or token".to_string();

    match response {
        Ok(artist) => {
            if artist.get("artistId").is_none() || artist["artistId"].as_str().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(artist))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Artist's details fetched successfully".to_string(),
                    Some(formatted_payload(artist, camel, &artist_payload)),
                )))
            }
        }

        Err(e) => {
            println!("Error: {e}");

            Err(err_msg)
        }
    }
}

/// Helper function to make request to `artist.getArtistMoreSong` endpoint of JioSaavn API to get artist songs
///
/// ## Arguments
///
/// * `artist_id` - Artist id
/// * `page` - Page number
/// * `category` - Category of songs `[latest, alphabetical]`
/// * `sort` - Sort order `[asc, desc]`
///
/// ## Returns
///
/// * `Result<ArtistSongsResponse, reqwest::Error>` - Result of artist songs payload
pub async fn get_artists_songs_albums(
    id: String,
    page: String,
    cat: String,
    sort: String,
    path: String,
    raw: bool,
    camel: bool,
) -> Result<RArtistSongsOrAlbums, String> {
    let endpoint = if path.as_str() == "songs" {
        ARTISTS_SONGS
    } else {
        ARTISTS_ALBUMS
    };

    let query = vec![
        ("artistId".to_string(), id),
        ("p".to_string(), page),
        ("category".to_string(), cat),
        ("sort_order".to_string(), sort),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(endpoint, true, Some(query)).await;

    let err_msg = format!("❌ Artist's top {path} not found, please check the id");

    match response {
        Ok(data) => {
            let is_empty = if path == "songs" {
                data["topSongs"]["songs"].as_array().unwrap().is_empty()
            } else {
                data["topAlbums"]["albums"].as_array().unwrap().is_empty()
            };

            if is_empty {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(data))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    format!("✅ Artist's {path} fetched successfully"),
                    Some(formatted_payload(
                        data,
                        camel,
                        &artist_songs_or_albums_payload,
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

/// Helper function to make request to `search.artistOtherTopSongs` endpoint of JioSaavn API to get artist recommended songs
///
/// ## Arguments
///
/// * `artist_id` - Artist id
/// * `song_id` - Song id
/// * `languages` - Comma separated languages
/// * Available languages: `hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,
///  `gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`,
///  `rajasthani`, `odia`, `assamese`
///
/// ## Returns
///
/// * `Result<Vec<SongResponse>, reqwest::Error>` - Result of artist recommended songs payload
#[allow(clippy::too_many_arguments)]
pub async fn get_artist_top_songs(
    artist_id: String,
    song_id: String,
    page: String,
    lang: String,
    cat: String,
    sort: String,
    raw: bool,
    camel: bool,
) -> Result<RArtistTopSongs, String> {
    let query = vec![
        ("artist_ids".to_string(), artist_id),
        ("song_id".to_string(), song_id),
        ("language".to_string(), lang),
        ("page".to_string(), page),
        ("category".to_string(), cat),
        ("sort_order".to_string(), sort),
    ]
    .into_iter()
    .collect();

    let response = http::<Value>(ARTISTS_TOP_SONGS, true, Some(query)).await;
    let err_msg = "❌ Artist's Top Songs not found, please check the ids".to_string();

    match response {
        Ok(songs) => {
            if songs.as_array().unwrap().is_empty() {
                Err(err_msg)
            } else if raw {
                Ok(Union::A(songs))
            } else {
                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Album details fetched successfully".to_string(),
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_get_artist_details_by_id() -> Result<(), reqwest::Error> {
//         let result = get_artist_details_by_id("568707").await?;

//         dbg!(result);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_artist_details_by_link() -> Result<(), reqwest::Error> {
//         let result =
//             get_artist_details_by_link("https://www.jiosaavn.com/artist/sia-/C4hxFiXrHws_").await?;

//         dbg!(result);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_artist_songs() -> Result<(), reqwest::Error> {
//         let result = get_artist_songs("568707", 1, "latest", "asc").await?;

//         dbg!(result);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_artist_albums() -> Result<(), reqwest::Error> {
//         let result = get_artist_albums("568707", 1, "latest", "asc").await?;

//         dbg!(result);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_artist_top_songs() -> Result<(), reqwest::Error> {
//         let result = get_artist_top_songs("459320", "_rJmbKSP", "english").await?;

//         dbg!(result);

//         Ok(())
//     }
// }
