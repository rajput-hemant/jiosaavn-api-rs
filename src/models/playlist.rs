use serde::{Deserialize, Serialize};

use super::{
    artist::{ArtistMiniRequest, ArtistMiniResponse},
    quality::Quality,
    song::{SongRequest, SongResponse},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
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
    pub list: Vec<SongRequest>,
    pub more_info: PlaylistRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRequestMoreInfo {
    pub uid: String,
    pub is_dolby_content: bool,
    pub last_updated: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub follower_count: String,
    pub fan_count: String,
    pub playlist_type: String,
    pub share: String,
    pub video_count: String,
    pub artists: Vec<ArtistMiniRequest>,
    pub subtitle_desc: Vec<String>,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResponse {
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
    pub explicit_content: bool,
    pub list_count: u64,
    pub list_type: String,
    pub user_id: String,
    pub is_dolby_content: bool,
    pub last_updated: u64,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub follower_count: u64,
    pub fan_count: u64,
    pub share: u64,
    pub video_count: u64,
    pub artists: Vec<ArtistMiniResponse>,
    pub subtitle_desc: Vec<String>,
    pub songs: Vec<SongResponse>,
}
