use serde::{Deserialize, Serialize};

use super::{artist::ArtistMap, misc::Rights, quality::Quality};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlbumSongList {
    String(String),
    List(Vec<AlbumSong>),
}

#[derive(Debug, Deserialize)]
pub struct AlbumRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub header_desc: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
    pub language: String,
    pub year: String,
    pub play_count: String,
    pub explicit_content: String,
    pub list_count: String,
    pub list_type: String,
    pub more_info: AlbumRequestMoreInfo,
    pub image: String,
    pub list: AlbumSongList,
}

#[derive(Debug, Deserialize)]
pub struct AlbumRequestMoreInfo {
    pub release_date: Option<String>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMap>,
    pub song_count: Option<String>,
    pub copyright_text: Option<String>,
    pub is_dolby_content: Option<bool>,
    pub meta_html: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumSong {
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
    pub more_info: SongMoreInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongMoreInfo {
    pub music: String,
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
    pub artist_map: ArtistMap,
    pub release_date: String,
    pub vcode: String,
    pub vlink: String,
    pub triller_available: bool,
    pub request_jiotune_flag: bool,
    pub webp: String,
    pub lyrics_id: Option<String>,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: String,
    pub play_count: u64,
    pub explicit: bool,
    pub year: u64,
    pub url: String,
    pub image: Quality,
    pub release_date: Option<String>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMap>,
    pub song_count: Option<u64>,
    pub copyright_text: Option<String>,
    pub is_dolby_content: Option<bool>,
    pub meta_html: Option<String>,
    pub songs: AlbumSongList,
}
