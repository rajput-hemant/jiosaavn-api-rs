use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::{
    album::{AlbumRequest, AlbumResponse},
    misc::{Quality, Union},
    response::CResponse,
    song::{SongRequest, SongResponse},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRequest {
    #[serde(rename = "artistId")]
    pub artist_id: String,
    pub name: String,
    pub subtitle: String,
    pub image: String,
    pub follower_count: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "dominantLanguage")]
    pub dominant_language: String,
    #[serde(rename = "dominantType")]
    pub dominant_type: String,
    #[serde(rename = "topSongs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_songs: Option<Vec<SongRequest>>,
    #[serde(rename = "topAlbums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_albums: Option<Vec<AlbumRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_artist_playlist: Option<Vec<ArtistPlaylistRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_artist_playlist: Option<Vec<ArtistPlaylistRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singles: Option<Vec<ArtistAlbumRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_release: Option<Vec<ArtistAlbumRequest>>,
    #[serde(rename = "similarArtists")]
    pub similar_artists: Vec<SimilarArtistRequest>,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: bool,
    pub bio: String,
    pub dob: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub urls: Urls,
    #[serde(rename = "availableLanguages")]
    pub available_languages: Vec<String>,
    pub fan_count: String,
    pub is_followed: bool,
    pub modules: ArtistModulesRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistAlbumRequest {
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
    pub play_count: Union<String, u64>,
    pub explicit_content: String,
    pub list_count: String,
    pub list_type: String,
    pub list: String,
    pub more_info: ArtistSongMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistSongMoreInfo {
    pub query: String,
    pub text: String,
    pub music: Option<String>,
    pub song_count: String,
    #[serde(rename = "artistMap")]
    pub artist_map: ArtistMapRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistModulesRequest {
    #[serde(rename = "topSongs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_songs: Option<ArtistModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_release: Option<ArtistModule>,
    #[serde(rename = "topAlbums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_albums: Option<ArtistModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_artist_playlist: Option<ArtistModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_artist_playlist: Option<ArtistModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singles: Option<ArtistModule>,
    #[serde(rename = "similarArtists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similar_artists: Option<ArtistModule>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtistModule {
    pub title: String,
    pub subtitle: String,
    pub position: u64,
    pub source: String,
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
pub struct ArtistPlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub explicit_content: String,
    pub more_info: ArtistPlaylistMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistPlaylistMoreInfo {
    pub uid: String,
    pub firstname: String,
    pub artist_name: Option<Vec<String>>,
    pub entity_type: String,
    pub entity_sub_type: String,
    pub video_available: bool,
    pub is_dolby_content: Option<bool>,
    pub sub_types: Option<String>,
    pub images: Option<String>,
    pub lastname: String,
    pub song_count: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSongsOrAlbumsRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_albums: Option<ArtistTopSongsOrAlbums<AlbumRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_songs: Option<ArtistTopSongsOrAlbums<SongRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistTopSongsOrAlbums<T> {
    pub total: u64,
    pub last_page: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub songs: Option<Vec<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<T>>,
}

/*---------------------- Response ---------------------- */
pub type RArtist = Union<Value, CResponse<Union<Value, ArtistResponse>>>;
pub type RArtistSongsOrAlbums = Union<Value, CResponse<Union<Value, ArtistSongsOrAlbumsResponse>>>;
pub type RArtistTopSongs = Union<Value, CResponse<Union<Value, Vec<SongResponse>>>>;

#[derive(Debug, Serialize, Deserialize)]
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
    pub dedicated_artist_playlist: Vec<ArtistPlaylistResponse>,
    pub featured_artist_playlist: Vec<ArtistPlaylistResponse>,
    pub singles: Vec<ArtistAlbumResponse>,
    pub latest_release: Vec<ArtistAlbumResponse>,
    pub similar_artists: Vec<SimilarArtistResponse>,
    pub is_radio_present: bool,
    pub bio: Vec<Bio>,
    pub dob: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub urls: Urls,
    pub available_languages: Vec<String>,
    pub fan_count: u64,
    pub is_followed: bool,
    pub modules: ArtistModulesResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistModulesResponse {
    pub top_songs: ArtistModule,
    pub latest_release: ArtistModule,
    pub top_albums: ArtistModule,
    pub dedicated_artist_playlist: ArtistModule,
    pub featured_artist_playlist: ArtistModule,
    pub singles: ArtistModule,
    pub similar_artists: ArtistModule,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistAlbumResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    pub header_desc: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub image: Quality,
    pub language: String,
    pub year: u64,
    pub play_count: u64,
    pub explicit: bool,
    pub list_count: u64,
    pub list_type: String,
    pub list: String,
    pub query: String,
    pub text: String,
    pub music: Option<String>,
    pub song_count: u64,
    pub artist_map: ArtistMapResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarArtistResponse {
    pub id: String,
    pub name: String,
    pub roles: HashMap<String, String>,
    pub aka: String,
    pub fb: String,
    pub twitter: String,
    pub wiki: String,
    pub similar: Vec<Similar>,
    pub dob: String,
    pub image_url: Quality,
    pub search_keywords: String,
    pub primary_artist_id: String,
    pub languages: HashMap<String, String>,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_radio_present: bool,
    pub dominant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Similar {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bio {
    pub title: String,
    pub text: String,
    pub sequence: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub image: Quality,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistPlaylistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub user_id: String,
    pub firstname: String,
    pub artist_name: Option<String>,
    pub entity_type: String,
    pub entity_sub_type: String,
    pub video_available: bool,
    pub is_dolby_content: Option<bool>,
    pub sub_types: Option<String>,
    pub images: Option<String>,
    pub lastname: String,
    pub song_count: u64,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistSongsOrAlbumsResponse {
    pub id: String,
    pub name: String,
    pub image: Quality,
    pub follower_count: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_verified: bool,
    pub dominant_language: String,
    pub dominant_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_songs: Option<ArtistTopSongsOrAlbums<SongResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_albums: Option<ArtistTopSongsOrAlbums<AlbumResponse>>,
}
