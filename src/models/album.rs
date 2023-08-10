use serde::{Deserialize, Serialize};

use super::{
    artist::{ArtistMapRequest, ArtistMapResponse},
    misc::Rights,
    quality::Quality,
    song::{SongRequest, SongResponse},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlbumSongList {
    String(String),
    List(Vec<SongRequest>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRequestMoreInfo {
    pub release_date: Option<String>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMapRequest>,
    pub song_count: Option<String>,
    pub copyright_text: Option<String>,
    pub is_dolby_content: Option<bool>,
    #[serde(rename = "320kbps")]
    pub _320kbps: Option<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub album_url: Option<String>,
    pub duration: Option<String>,
    pub encrypted_media_url: Option<String>,
    pub has_lyrics: Option<String>,
    pub label: Option<String>,
    pub label_url: Option<String>,
    pub lyrics_snippet: Option<String>,
    pub music: Option<String>,
    pub origin: Option<String>,
    pub rights: Option<Rights>,
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
    pub artist_map: Option<ArtistMapResponse>,
    pub song_count: Option<u64>,
    pub copyright_text: Option<String>,
    pub is_dolby_content: Option<bool>,
    pub songs: Vec<SongResponse>,
}
