use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{misc::Rights, quality::Quality};

#[derive(Debug, Deserialize)]
pub struct PlaylistRequest {
    pub listid: String,
    pub listname: String,
    pub perma_url: String,
    pub follower_count: String,
    pub uid: String,
    pub last_updated: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub image: String,
    pub share: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub list_count: String,
    pub fan_count: String,
    pub is_dolby_playlist: bool,
    pub video_available: bool,
    pub video_count: u64,
    pub meta_html: Option<String>,
    pub songs: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub song: String,
    pub album: String,
    pub year: String,
    pub music: String,
    pub music_id: String,
    pub primary_artists: String,
    pub primary_artists_id: String,
    pub featured_artists: String,
    pub featured_artists_id: String,
    pub singers: String,
    pub starring: String,
    pub image: String,
    pub label: String,
    pub albumid: String,
    pub language: String,
    pub origin: String,
    pub play_count: String,
    pub copyright_text: String,
    #[serde(rename = "320kbps")]
    pub _320kbps: String,
    pub is_dolby_content: bool,
    pub explicit_content: u64,
    pub has_lyrics: String,
    pub lyrics_snippet: String,
    pub encrypted_media_url: String,
    pub encrypted_media_path: String,
    pub media_preview_url: String,
    pub perma_url: String,
    pub album_url: String,
    pub duration: String,
    pub rights: Rights,
    #[serde(rename = "artistMap")]
    pub artist_map: HashMap<String, String>,
    pub release_date: String,
    pub triller_available: bool,
    pub label_url: String,
    pub vcode: Option<String>,
    pub vlink: Option<String>,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResponse {
    pub id: String,
    pub name: String,
    pub url: String,
    pub follower_count: u64,
    pub user_id: String,
    pub last_updated: u64,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub image: Quality,
    pub share: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub list_count: u64,
    pub fan_count: u64,
    pub is_dolby_playlist: bool,
    pub video_available: bool,
    pub video_count: u64,
    pub meta_html: Option<String>,
    pub songs: Vec<Song>,
}
