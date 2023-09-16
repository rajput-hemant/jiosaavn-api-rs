use super::{
    artist::{ArtistMiniRequest, ArtistMiniResponse},
    misc::{Quality, Union},
    response::CResponse,
    song::{SongRequest, SongResponse},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
    pub header_desc: String,
    pub image: String,
    pub language: String,
    pub year: Option<String>,
    pub play_count: Option<String>,
    pub explicit_content: String,
    pub list_count: Option<String>,
    pub list_type: String,
    pub list: Option<Union<String, Vec<SongRequest>>>,
    pub more_info: PlaylistRequestMoreInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<PlaylistModulesRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRequestMoreInfo {
    pub uid: String,
    pub is_dolby_content: bool,
    pub last_updated: Option<String>,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub follower_count: Option<String>,
    pub fan_count: Option<String>,
    pub playlist_type: String,
    pub share: Option<String>,
    pub video_count: Option<String>,
    pub artists: Option<Vec<ArtistMiniRequest>>,
    pub subtitle_desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesRequest {
    #[serde(rename = "relatedPlaylist")]
    pub related_playlist: PlaylistModulesRelatedPlaylistRequest,
    #[serde(rename = "currentlyTrendingPlaylists")]
    pub currently_trending_playlists: PlaylistModulesCurrentlyTrendingPlaylistsRequest,
    pub artists: PlaylistModulesPlaylistArtistsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesRelatedPlaylistRequest {
    pub source: String,
    pub position: u32,
    pub title: String,
    pub subtitle: String,
    pub source_params: PlaylistModulesRelatesParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesCurrentlyTrendingPlaylistsRequest {
    pub source: String,
    pub position: u32,
    pub title: String,
    pub subtitle: String,
    pub source_params: PlaylistModulesEntityParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesPlaylistArtistsRequest {
    pub source: String,
    pub position: u32,
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesRelatesParamsRequest {
    pub listid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesEntityParamsRequest {
    pub entity_type: String,
    pub entity_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRecRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub explicit_content: String,
    pub more_info: PlaylistRecMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRecMoreInfo {
    pub firstname: String,
}

/*---------------------- Response ---------------------- */
pub type RPlaylist = Union<Value, CResponse<PlaylistResponse>>;

pub type RPlaylistRec = Union<Value, CResponse<Vec<PlaylistRecResponse>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub header_desc: String,
    pub image: Quality,
    pub language: String,
    pub year: Option<u64>,
    pub play_count: Option<u64>,
    pub explicit: bool,
    pub list_count: Option<u64>,
    pub list_type: String,
    pub user_id: String,
    pub is_dolby_content: bool,
    pub last_updated: Option<u64>,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub follower_count: Option<u64>,
    pub fan_count: Option<u64>,
    pub share: Option<u64>,
    pub video_count: Option<u64>,
    pub subtitle_desc: Vec<String>,
    pub artists: Option<Vec<ArtistMiniResponse>>,
    pub songs: Option<Vec<SongResponse>>,
    pub modules: Option<PlaylistModulesResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesResponse {
    pub related_playlist: PlaylistModulesRelatedResponse,
    pub currently_trending_playlists: PlaylistModulesCurrentlyTrendingPlaylistsResponse,
    pub artists: PlaylistModulesArtistsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesRelatedResponse {
    pub source: String,
    pub position: u32,
    pub title: String,
    pub subtitle: String,
    pub params: PlaylistModulesRelatedParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesCurrentlyTrendingPlaylistsResponse {
    pub source: String,
    pub position: u32,
    pub title: String,
    pub subtitle: String,
    pub params: PlaylistModulesTrendingPlaylistsParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesArtistsResponse {
    pub source: String,
    pub position: u32,
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesRelatedParamsResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistModulesTrendingPlaylistsParamsResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRecResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub firstname: String,
}
