use serde::Deserialize;

use super::album::{AlbumRequest, ArtistMap};

#[derive(Debug, Deserialize)]
pub struct ModulesRequest {
    pub radio: Radio,
    pub browse_discover: Vec<BrowseDiscover>,
    pub new_albums: Vec<AlbumRequest>,
    pub charts: Vec<Chart>,
    pub top_shows: TopShows,
    pub new_trending: Vec<NewTrending>,
    pub top_playlists: Vec<TopPlaylist>,
}

#[derive(Debug, Deserialize)]
pub struct Radio {
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
    pub more_info: FeaturedStationMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct FeaturedStationMoreInfo {
    pub description: Option<String>,
    pub featured_station_type: String,
    pub query: String,
    pub color: Option<String>,
    pub language: String,
    pub station_display_text: String,
}

#[derive(Debug, Deserialize)]
pub struct BrowseDiscover {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: BrowseDiscoverMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct BrowseDiscoverMoreInfo {
    pub badge: String,
    pub sub_type: String,
    pub available: String,
    pub is_featured: String,
    pub tags: serde_json::Value, // You can use a specific type if you know the expected structure of "tags".
    pub video_url: String,
    pub video_thumbnail: String,
}

#[derive(Debug, Deserialize)]
pub struct Chart {
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
pub struct TopShows {
    pub badge: String,
    pub shows: Vec<Show>,
    pub last_page: bool,
}

#[derive(Debug, Deserialize)]
pub struct Show {
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
pub struct NewTrending {
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
    pub more_info: NewTrendingMoreInfo,
}

#[derive(Debug, Deserialize)]
pub struct NewTrendingMoreInfo {
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
pub struct Rights {
    pub code: String,
    pub cacheable: String,
    pub delete_cached_object: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TopPlaylist {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: TopPlaylistMoreInfo,
    pub explicit_content: String,
    pub mini_obj: bool,
}

#[derive(Debug, Deserialize)]
pub struct TopPlaylistMoreInfo {
    pub song_count: String,
    pub firstname: String,
    pub follower_count: String,
    pub last_updated: String,
    pub uid: String,
}
