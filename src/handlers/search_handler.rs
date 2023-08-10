use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        album::AlbumResponse,
        response::{CustomResponse, StatusCode},
        search::{
            AllSearchResponse, SearchArtistResponse, SearchPlaylistResponse, TSearchResponse,
            TopSearchesResponse,
        },
        song::SongResponse,
    },
    services::search_service::{
        get_top_searches, search_albums, search_all, search_artists, search_playlists, search_songs,
    },
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    query: Option<String>,
    page: Option<u64>,
    limit: Option<u64>,
}

/// Handler for `/search/top` route
///
/// ## Returns
///
/// * `Json<CustomResponse<Vec<TopSearchesResponse>>>` - Json response
pub async fn top_searches_handler() -> Json<CustomResponse<Vec<TopSearchesResponse>>> {
    let result = get_top_searches().await;

    match result {
        Ok(result) => Json(CustomResponse::new(
            StatusCode::Success,
            "✅ Top searches fetched successfully!",
            Some(result),
        )),
        Err(_) => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Failed to fetch top searches!",
            Some(vec![]),
        )),
    }
}

/// Handler for `/search` or `/search/all` route
///
/// ## Arguments
///
/// * `query` - search query
///
/// ## Returns
///
/// * `Json<CustomResponse<AllSearchResponse>>` - Json response
pub async fn search_all_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<AllSearchResponse>> {
    match params.query {
        Some(query) => {
            let result = search_all(&query).await;

            match result {
                Ok(result) => Json(CustomResponse::new(
                    StatusCode::Success,
                    "✅ Search results fetched successfully!",
                    Some(result),
                )),
                Err(_) => Json(CustomResponse::new(
                    StatusCode::Failed,
                    "❌ Failed to fetch search results!",
                    None,
                )),
            }
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Query is required!",
            None,
        )),
    }
}

/// Handler for `/search/songs` route
///
/// ## Arguments
///
/// * `query` - search query
/// * `page` - page number
/// * `limit` - songs limit per page
///
/// ## Returns
///
/// * `Json<CustomResponse<TSearchResponse<SongResponse>>>` - Json response
pub async fn songs_search_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<TSearchResponse<SongResponse>>> {
    match params.query {
        Some(query) => {
            let (page, limit) = get_default_params(params.page, params.limit);

            let result = search_songs(&query, page, limit).await;

            match result {
                Ok(result) => Json(CustomResponse::new(
                    StatusCode::Success,
                    "✅ Search results for songs fetched successfully!",
                    Some(result),
                )),
                Err(_) => Json(CustomResponse::new(
                    StatusCode::Failed,
                    "❌ Failed to fetch songs search results!",
                    None,
                )),
            }
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Query is required!",
            None,
        )),
    }
}

/// Handler for `/search/albums` route
///
/// ## Arguments
///
/// * `query` - search query
/// * `page` - page number
/// * `limit` - albums limit per page
///
/// ## Returns
///
/// * `Json<CustomResponse<TSearchResponse<AlbumResponse>>>` - Json response
pub async fn albums_search_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<TSearchResponse<AlbumResponse>>> {
    match params.query {
        Some(query) => {
            let (page, limit) = get_default_params(params.page, params.limit);

            let result = search_albums(&query, page, limit).await;

            match result {
                Ok(result) => Json(CustomResponse::new(
                    StatusCode::Success,
                    "✅ Search results for albums fetched successfully!",
                    Some(result),
                )),
                Err(_) => Json(CustomResponse::new(
                    StatusCode::Failed,
                    "❌ Failed to fetch albums search results!",
                    None,
                )),
            }
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Query is required!",
            None,
        )),
    }
}

/// Handler for `/search/playlists` route
///
/// ## Arguments
///
/// * `query` - search query
/// * `page` - page number
/// * `limit` - playlists limit per page
///
/// ## Returns
///
/// * `Json<CustomResponse<TSearchResponse<SearchPlaylistResponse>>>` - Json response
pub async fn playlists_search_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<TSearchResponse<SearchPlaylistResponse>>> {
    match params.query {
        Some(query) => {
            let (page, limit) = get_default_params(params.page, params.limit);

            let result = search_playlists(&query, page, limit).await;

            match result {
                Ok(result) => Json(CustomResponse::new(
                    StatusCode::Success,
                    "✅ Search results for playlists fetched successfully!",
                    Some(result),
                )),
                Err(_) => Json(CustomResponse::new(
                    StatusCode::Failed,
                    "❌ Failed to fetch playlists search results!",
                    None,
                )),
            }
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Query is required!",
            None,
        )),
    }
}

/// Handler for `/search/artists` route
///
/// ## Arguments
///
/// * `query` - search query
/// * `page` - page number
/// * `limit` - artists limit per page
///
/// ## Returns
///
/// * `Json<CustomResponse<TSearchResponse<SearchArtistResponse>>>` - Json response
pub async fn artists_search_handler(
    Query(params): Query<Params>,
) -> Json<CustomResponse<TSearchResponse<SearchArtistResponse>>> {
    match params.query {
        Some(query) => {
            let (page, limit) = get_default_params(params.page, params.limit);

            let result = search_artists(&query, page, limit).await;

            match result {
                Ok(result) => Json(CustomResponse::new(
                    StatusCode::Success,
                    "✅ Search results for artists fetched successfully!",
                    Some(result),
                )),
                Err(_) => Json(CustomResponse::new(
                    StatusCode::Failed,
                    "❌ Failed to fetch artists search results!",
                    None,
                )),
            }
        }
        None => Json(CustomResponse::new(
            StatusCode::Failed,
            "❌ Query is required!",
            None,
        )),
    }
}

fn get_default_params(page: Option<u64>, limit: Option<u64>) -> (u64, u64) {
    match (page, limit) {
        (Some(page), Some(limit)) => (page, limit),
        (Some(page), None) => (page, 10),
        (None, Some(limit)) => (1, limit),
        (None, None) => (1, 10),
    }
}
