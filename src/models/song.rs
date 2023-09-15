use super::{
    artist::{ArtistMapRequest, ArtistMapResponse},
    misc::{Quality, Rights, Union},
    response::CResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SongObjRequest {
    pub songs: Vec<SongRequest>,
    pub modules: Option<SongModulesRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub header_desc: String,
    pub perma_url: String,
    pub image: String,
    pub language: String,
    pub year: String,
    pub play_count: Union<String, u64>,
    pub explicit_content: String,
    pub list_count: String,
    pub list_type: String,
    pub list: String,
    pub more_info: SongRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRequestMoreInfo {
    pub music: String,
    pub album_id: String,
    pub album: String,
    pub label: String,
    pub label_url: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesRequest {
    pub reco: SongModulesRecoRequest,
    #[serde(rename = "currentlyTrending")]
    pub currently_trending: SongModulesCurrentlyTrendingRequest,
    #[serde(rename = "songsBysameArtists")]
    pub songs_by_same_artists: SongModulesSongsBySameArtistsRequest,
    #[serde(rename = "songsBysameActors")]
    pub songs_by_same_actors: SongModulesSongsBySameActorsRequest,
    pub artists: SongModulesArtistsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesRecoRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: SongModulesRecoParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesCurrentlyTrendingRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: SongModulesEntityParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesSongsBySameArtistsRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: SongModulesArtistSongParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesSongsBySameActorsRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub source_params: SongModulesActorSongParamsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesArtistsRequest {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesRecoParamsRequest {
    pub pid: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesEntityParamsRequest {
    pub entity_type: String,
    pub entity_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesArtistSongParamsRequest {
    pub artist_ids: String,
    pub song_id: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesActorSongParamsRequest {
    pub actor_ids: String,
    pub song_id: String,
    pub language: String,
}

/*---------------------- Response ---------------------- */
pub type RSong = Union<Value, CResponse<SongObjResponse>>;
pub type RSongReco = Union<Value, CResponse<Vec<SongResponse>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SongObjResponse {
    pub songs: Vec<SongResponse>,
    pub modules: Option<SongModulesResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongResponse {
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
    pub music: String,
    pub album: String,
    pub album_id: String,
    pub album_url: String,
    pub label: String,
    pub label_url: String,
    pub origin: String,
    pub is_dolby_content: bool,
    #[serde(rename = "320kbps")]
    pub _320kbps: bool,
    pub download_url: Quality,
    pub duration: u64,
    pub rights: Rights,
    pub has_lyrics: bool,
    pub lyrics_id: Option<String>,
    pub lyrics_snippet: String,
    pub starred: bool,
    pub copyright_text: String,
    #[serde(rename = "artistMap")]
    pub artist_map: ArtistMapResponse,
    pub release_date: Option<String>,
    pub vcode: Option<String>,
    pub vlink: Option<String>,
    pub triller_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesResponse {
    pub recommend: SongModulesRecommendResponse,
    pub currently_trending: SongModulesCurrentlyTrendingResponse,
    pub songs_by_same_artists: SongModulesSongsBySameArtistsResponse,
    pub songs_by_same_actors: SongModulesSongsBySameActorsResponse,
    pub artists: SongModulesArtistsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesRecommendResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: SongModulesRecommendParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesCurrentlyTrendingResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: SongModulesTrendingParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesSongsBySameArtistsResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: SongModulesArtistSongParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesSongsBySameActorsResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
    pub params: SongModulesActorSongParamsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesArtistsResponse {
    pub title: String,
    pub subtitle: String,
    pub source: String,
    pub position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesRecommendParamsResponse {
    pub id: String,
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesTrendingParamsResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesArtistSongParamsResponse {
    pub artist_id: String,
    pub song_id: String,
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongModulesActorSongParamsResponse {
    pub actor_id: String,
    pub song_id: String,
    pub lang: String,
}
