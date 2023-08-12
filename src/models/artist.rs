use serde::{Deserialize, Serialize};

use super::{
    album::{AlbumRequest, AlbumResponse},
    quality::Quality,
    song::{SongRequest, SongResponse},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRequest {
    pub artist_id: String,
    pub name: String,
    pub subtitle: String,
    pub image: String,
    #[serde(rename = "follower_count")]
    pub follower_count: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    pub top_songs: Vec<SongRequest>,
    pub top_albums: Vec<AlbumRequest>,
    pub similar_artists: Vec<SimilarArtistRequest>,
    pub is_radio_present: bool,
    pub bio: String,
    pub dob: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub urls: Urls,
    pub available_languages: Vec<String>,
    #[serde(rename = "fan_count")]
    pub fan_count: String,
    #[serde(rename = "is_followed")]
    pub is_followed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarArtistRequest {
    pub id: String,
    pub name: String,
    pub roles: String,
    pub aka: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub similar: String,
    pub dob: String,
    pub image_url: String,
    pub search_keywords: String,
    pub primary_artist_id: String,
    pub combine_artist_pages: u64,
    pub replace_with_primary_artists: u64,
    pub languages: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: bool,
    #[serde(rename = "dominantType")]
    pub dominant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    pub albums: String,
    pub bio: String,
    pub comments: String,
    pub songs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistMapRequest {
    pub primary_artists: Vec<ArtistMiniRequest>,
    pub featured_artists: Vec<ArtistMiniRequest>,
    pub artists: Vec<ArtistMiniRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistMiniRequest {
    pub id: String,
    pub name: String,
    pub role: String,
    pub image: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSongsRequest {
    pub artist_id: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "follower_count")]
    pub follower_count: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    pub top_songs: ArtistTopSongs<SongRequest>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbumsRequest {
    pub artist_id: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "follower_count")]
    pub follower_count: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    pub top_albums: ArtistTopAlbums<AlbumRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistTopSongs<T> {
    pub total: u64,
    pub last_page: bool,
    pub songs: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistTopAlbums<T> {
    pub total: u64,
    pub last_page: bool,
    pub albums: Vec<T>,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    pub image: Quality,
    pub follower_count: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    pub top_songs: Vec<SongResponse>,
    pub top_albums: Vec<AlbumResponse>,
    pub similar_artists: Vec<SimilarArtistResponse>,
    pub is_radio_present: bool,
    pub bio: String,
    pub dob: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub urls: Urls,
    pub available_languages: Vec<String>,
    pub fan_count: u64,
    pub is_followed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarArtistResponse {
    pub id: String,
    pub name: String,
    pub roles: String,
    pub aka: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub similar: String,
    pub dob: String,
    pub image_url: Quality,
    pub search_keywords: String,
    pub primary_artist_id: String,
    pub combine_artist_pages: u64,
    pub replace_with_primary_artists: u64,
    pub languages: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_radio_present: bool,
    pub dominant_type: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistMapResponse {
    pub primary_artists: Vec<ArtistMiniResponse>,
    pub featured_artists: Vec<ArtistMiniResponse>,
    pub artists: Vec<ArtistMiniResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistMiniResponse {
    pub id: String,
    pub name: String,
    pub role: String,
    pub image: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSongsResponse {
    pub id: String,
    pub name: String,
    pub image: Quality,
    pub follower_count: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    pub top_songs: ArtistTopSongs<SongResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbumsResponse {
    pub id: String,
    pub name: String,
    pub image: Quality,
    pub follower_count: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    pub top_albums: ArtistTopAlbums<AlbumResponse>,
}
