use serde::{Deserialize, Serialize};

use super::{
    artist::ArtistMap,
    quality::Quality,
    song::{SongRequest, SongResponse},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlbumSongList {
    String(String),
    List(Vec<SongRequest>),
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
    pub songs: Vec<SongResponse>,
}
