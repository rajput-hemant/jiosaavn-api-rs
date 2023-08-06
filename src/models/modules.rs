use serde::{Deserialize, Serialize};

use super::{
    album::{AlbumRequest, AlbumResponse},
    artist::ArtistMap,
    misc::Rights,
    quality::Quality,
};

#[derive(Debug, Deserialize)]
pub struct ModulesRequest {
    pub radio: RadioRequest,
    pub browse_discover: Vec<DiscoverRequest>,
    pub new_albums: Vec<AlbumRequest>,
    pub charts: Vec<ChartRequest>,
    pub top_shows: TopShowsRequest,
    pub new_trending: Vec<TrendingRequest>,
    pub top_playlists: Vec<ModulePlaylistRequest>,
}

#[derive(Debug, Deserialize)]
pub struct RadioRequest {
    pub featured_stations: Vec<FeaturedStation>,
}

#[derive(Debug, Deserialize)]
pub struct FeaturedStation {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: RadioMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct RadioMoreInfo {
    pub description: Option<String>,
    pub featured_station_type: String,
    pub query: String,
    pub color: Option<String>,
    pub language: String,
    pub station_display_text: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: DiscoverMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverMoreInfo {
    pub badge: String,
    pub sub_type: String,
    pub available: String,
    pub is_featured: String,
    pub video_url: String,
    pub video_thumbnail: String,
}

#[derive(Debug, Deserialize)]
pub struct ChartRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: ChartMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct ChartMoreInfo {
    pub firstname: String,
}

#[derive(Debug, Deserialize)]
pub struct TopShowsRequest {
    pub shows: Vec<ShowRequest>,
    pub last_page: bool,
}

#[derive(Debug, Deserialize)]
pub struct ShowRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: ShowMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct ShowMoreInfo {
    pub season_number: String,
    pub release_date: String,
    pub year: String,
    pub badge: String,
    pub square_image: String,
}

#[derive(Debug, Deserialize)]
pub struct TrendingRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub header_desc: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
    pub image: String,
    pub language: String,
    pub year: String,
    pub play_count: String,
    pub explicit_content: String,
    pub list_count: String,
    pub list_type: String,
    pub list: String,
    pub more_info: TrendingMoreInfo,
}

#[derive(Debug, Deserialize)]
pub struct TrendingMoreInfo {
    pub release_date: Option<String>,
    pub song_count: Option<String>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMap>,
    pub music: Option<String>,
    pub album_id: Option<String>,
    pub album: Option<String>,
    pub label: Option<String>,
    pub origin: Option<String>,
    pub is_dolby_content: Option<bool>,
    #[serde(rename = "320kbps")]
    pub _320kbps: Option<String>,
    pub encrypted_media_url: Option<String>,
    pub encrypted_cache_url: Option<String>,
    pub album_url: Option<String>,
    pub duration: Option<String>,
    pub rights: Option<Rights>,
    pub cache_state: Option<String>,
    pub has_lyrics: Option<String>,
    pub lyrics_snippet: Option<String>,
    pub starred: Option<String>,
    pub copyright_text: Option<String>,
    pub is_ringtone_available: Option<bool>,
    pub label_url: Option<String>,
    pub vcode: Option<String>,
    pub vlink: Option<String>,
    pub triller_available: Option<bool>,
    pub request_jiotune_flag: Option<bool>,
    pub webp: Option<String>,
    pub lyrics_id: Option<String>,
    #[serde(rename = "isWeekly")]
    pub is_weekly: Option<String>,
    pub firstname: Option<String>,
    pub follower_count: Option<String>,
    pub fan_count: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModulePlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: PlaylistMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistMoreInfo {
    pub song_count: String,
    pub firstname: String,
    pub follower_count: String,
    pub last_updated: String,
    pub uid: String,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModulesResponse {
    pub radio: Vec<RadioResponse>,
    pub discover: Vec<DiscoverResponse>,
    pub albums: Vec<AlbumResponse>,
    pub charts: Vec<ChartResponse>,
    pub shows: TopShowResponse,
    pub trending: Vec<TrendingResponse>,
    pub playlists: Vec<ModulePlaylistResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub image: Quality,
    pub explicit: bool,
    pub language: String,
    pub description: Option<String>,
    pub query: String,
    pub color: Option<String>,
    pub featured_station_type: String,
    pub station_display_text: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub genre: String,
    pub available: String,
    pub is_featured: String,
    pub video_url: String,
    pub video_thumbnail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChartResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopShowResponse {
    pub shows: Vec<ShowResponse>,
    pub last_page: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub season_number: u64,
    pub release_date: String,
    pub year: u64,
    pub square_image: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub image: Quality,
    pub language: String,
    pub year: u64,
    pub play_count: u64,
    pub explicit: bool,
    pub release_date: Option<String>,
    pub song_count: Option<u64>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMap>,
    pub firstname: Option<String>,
    pub follower_count: Option<u64>,
    pub fan_count: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModulePlaylistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub song_count: u64,
    pub firstname: String,
    pub follower_count: u64,
    pub last_updated: u64,
    pub user_id: String,
}
