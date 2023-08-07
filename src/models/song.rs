use serde::{Deserialize, Serialize};

use super::{
    artist::{ArtistMapRequest, ArtistMapResponse},
    misc::Rights,
    quality::Quality,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub id: String,
    pub title: Option<String>,
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
    pub more_info: SongRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRequestMoreInfo {
    pub music: String,
    pub song: Option<String>,
    pub album_id: String,
    pub album: String,
    pub label: String,
    pub origin: String,
    pub is_dolby_content: bool,
    #[serde(rename = "320kbps")]
    pub _320kbps: String,
    pub encrypted_media_url: String,
    pub encrypted_cache_url: String,
    pub album_url: String,
    pub duration: String,
    pub rights: Rights,
    pub cache_state: String,
    pub has_lyrics: String,
    pub lyrics_snippet: String,
    pub starred: String,
    pub copyright_text: String,
    #[serde(rename = "artistMap")]
    pub artist_map: ArtistMapRequest,
    pub release_date: Option<String>,
    pub vcode: Option<String>,
    pub vlink: Option<String>,
    pub triller_available: bool,
    pub webp: String,
    pub lyrics_id: Option<String>,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongResponse {
    pub id: String,
    pub name: Option<String>,
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
    pub list: String,
    pub music: String,
    pub song: Option<String>,
    pub album_id: String,
    pub album: String,
    pub label: String,
    pub origin: String,
    pub is_dolby_content: bool,
    #[serde(rename = "320kbps")]
    pub _320kbps: bool,
    pub encrypted_media_url: String,
    pub encrypted_cache_url: String,
    pub album_url: String,
    pub duration: u64,
    pub rights: Rights,
    pub cache_state: String,
    pub has_lyrics: bool,
    pub lyrics_snippet: String,
    pub starred: bool,
    pub copyright_text: String,
    #[serde(rename = "artistMap")]
    pub artist_map: ArtistMapResponse,
    pub release_date: String,
    pub vcode: Option<String>,
    pub vlink: Option<String>,
    pub triller_available: bool,
    pub lyrics_id: Option<String>,
}
