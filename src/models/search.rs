use serde::{Deserialize, Serialize};

use super::quality::Quality;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest<T> {
    pub position: u64,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllSearchRequest {
    pub topquery: SearchRequest<TopQueryRequest>,
    pub albums: SearchRequest<SearchAlbumRequest>,
    pub songs: SearchRequest<SearchSongRequest>,
    pub playlists: SearchRequest<SearchPlaylistRequest>,
    pub artists: SearchRequest<SearchArtistRequest>,
    pub shows: SearchRequest<SearchShowRequest>,
    // pub episodes: SearchRequest<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: SearchAlbumRequestMoreInfo,
    pub explicit_content: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumRequestMoreInfo {
    pub music: String,
    pub year: String,
    pub is_movie: String,
    pub language: String,
    pub song_pids: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSongRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: SearchSongRequestMoreInfo,
    pub explicit_content: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSongRequestMoreInfo {
    pub ctr: u64,
    pub score: String,
    pub primary_artists: String,
    pub singers: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchArtistRequest {
    pub id: String,
    pub title: Option<String>,
    pub name: Option<String>,
    pub image: String,
    pub extra: Option<String>,
    pub role: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: bool,
    pub entity: u64,
    pub description: Option<String>,
    pub position: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchShowRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: SearchShowRequestMoreInfo,
    pub explicit_content: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchShowRequestMoreInfo {
    pub season_number: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopQueryRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: TopQueryRequestMoreInfo,
    pub explicit_content: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopQueryRequestMoreInfo {
    pub music: Option<String>,
    pub year: Option<String>,
    pub is_movie: Option<String>,
    pub language: String,
    pub song_pids: Option<String>,
    pub score: Option<String>,
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: SearchPlaylistRequestMoreInfo,
    pub explicit_content: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistRequestMoreInfo {
    pub uid: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub language: String,
    pub entity_type: Option<String>,
    pub song_count: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopSearchesRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub explicit_content: String,
    pub mini_obj: bool,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse<T> {
    pub position: u64,
    pub results: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TSearchResponse<T> {
    pub total: u64,
    pub start: u64,
    pub results: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllSearchResponse {
    pub topquery: SearchResponse<TopQueryRespone>,
    pub albums: SearchResponse<SearchAlbumResponse>,
    pub songs: SearchResponse<SearchSongResponse>,
    pub playlists: SearchResponse<SearchPlaylistResponse>,
    pub artists: SearchResponse<SearchArtistResponse>,
    pub shows: SearchResponse<SearchShowResponse>,
    // pub episodes: SearchResponse<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAlbumResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub description: String,
    pub music: String,
    pub year: u64,
    pub is_movie: bool,
    pub language: String,
    pub song_ids: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSongResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub description: String,
    pub score: f64,
    pub primary_artists: String,
    pub singers: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArtistResponse {
    pub id: String,
    pub name: Option<String>,
    pub image: String,
    pub extra: Option<String>,
    pub role: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: bool,
    pub entity: u64,
    pub description: Option<String>,
    pub position: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchShowResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub description: String,
    pub season_number: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopQueryRespone {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub description: String,
    pub music: Option<String>,
    pub year: Option<String>,
    pub is_movie: Option<String>,
    pub language: String,
    pub song_ids: Option<String>,
    pub score: Option<String>,
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub description: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub language: String,
    pub user_id: Option<String>,
    pub entity_type: Option<String>,
    pub song_count: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopSearchesResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
}
