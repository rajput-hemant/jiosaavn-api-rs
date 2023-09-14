use super::{
    artist::{ArtistMapRequest, ArtistMapResponse},
    misc::{Quality, Union},
    song::{SongRequest, SongResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
    pub header_desc: String,
    pub image: String,
    pub language: String,
    pub year: String,
    pub play_count: String,
    pub explicit_content: String,
    pub list_count: String,
    pub list_type: String,
    pub list: Option<Union<String, Vec<SongRequest>>>,
    pub more_info: AlbumRequestMoreInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<AlbumModulesRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRequestMoreInfo {
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMapRequest>,
    pub song_count: Option<String>,
    pub copyright_text: Option<String>,
    pub is_dolby_content: Option<bool>,
    pub label_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesRequest {
    pub reco: AlbumModulesRecoRequest,
    #[serde(rename = "currentlyTrending")]
    pub currently_trending: AlbumModulesCurrentlyTrendingRequest,
    #[serde(rename = "topAlbumsFromSameYear")]
    pub top_albums_from_same_year: AlbumModulesTopAlbumsFromSameYearRequest,
    pub artists: AlbumModulesArtistsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesRecoRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: AlbumRecoParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesCurrentlyTrendingRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: AlbumEntityParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesTopAlbumsFromSameYearRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: AlbumYearLangParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesArtistsRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRecoParamsRequest {
    pub albumid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumEntityParamsRequest {
    pub entity_type: String,
    pub entity_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumYearLangParamsRequest {
    pub album_year: String,
    pub album_lang: String,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub header_desc: String,
    pub language: String,
    pub play_count: u64,
    pub explicit: bool,
    pub year: u64,
    pub list_count: u64,
    pub list_type: String,
    pub url: String,
    pub image: Quality,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMapResponse>,
    pub song_count: Option<u64>,
    pub label_url: Option<String>,
    pub copyright_text: Option<String>,
    pub is_dolby_content: Option<bool>,
    pub songs: Option<Vec<SongResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<AlbumModulesResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesResponse {
    pub recommend: AlbumModulesRecommendResponse,
    pub currently_trending: AlbumModulesCurrentlyTrendingResponse,
    pub top_albums_from_same_year: AlbumModulesTopAlbumsFromSameYearResponse,
    pub artists: AlbumModulesArtistsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesRecommendResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: AlbumRecommendParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesCurrentlyTrendingResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: AlbumTrendingParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesTopAlbumsFromSameYearResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: AlbumYearLangParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumModulesArtistsResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRecommendParamsResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumTrendingParamsResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumYearLangParamsResponse {
    pub year: String,
    pub lang: String,
}
