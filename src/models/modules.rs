use super::{
    album::{AlbumRequest, AlbumResponse},
    get::{
        ChartRequest, ChartResponse, RadioRequest, RadioResponse, TrendingRequest, TrendingResponse,
    },
    misc::{Quality, Union},
    response::CResponse,
    song::{SongRequest, SongResponse},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesRequest {
    pub new_albums: ModuleAlbumRequest,
    pub artist_recos: Option<Vec<ArtistRecoRequest>>,
    pub browse_discover: Vec<DiscoverRequest>,
    pub charts: Vec<ChartRequest>,
    pub city_mod: Option<Vec<CityModRequest>>,
    pub global_config: GlobalConfig,
    pub modules: ModuleRequest,
    pub new_trending: TrendingRequest,
    pub radio: Vec<RadioRequest>,
    pub tag_mixes: Option<Vec<TagMixRequest>>,
    pub top_playlists: Vec<ModulesPlaylistRequest>,
    #[serde(flatten)]
    pub promos: HashMap<String, Vec<PromoRequest>>,
}

pub type ModuleAlbumRequest = Vec<Union<AlbumRequest, SongRequest>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRecoRequest {
    pub id: String,
    pub image: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
    pub explicit_content: String,
    pub more_info: ArtistRecoRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRecoRequestMoreInfo {
    pub featured_station_type: String,
    pub query: String,
    pub station_display_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: DiscoverMoreInfo,
    pub explicit_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverMoreInfo {
    pub badge: String,
    pub tags: Tags,
    pub sub_type: String,
    pub available: String,
    pub is_featured: String,
    pub video_url: String,
    pub video_thumbnail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub mood: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub situation: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub seasonality: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub genre: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityModRequest {
    pub id: String,
    pub image: String,
    pub title: String,
    pub subtitle: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub explicit_content: String,
    pub more_info: CityModRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityModRequestMoreInfo {
    pub featured_station_type: Option<String>,
    pub query: Option<String>,
    pub album_id: Option<String>,
    #[serde(default)]
    pub multiple_tunes: Option<Vec<CityModRequestMultipleTune>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityModRequestMultipleTune {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub subtype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub random_songs_listid: GlobalConfigItem,
    pub weekly_top_songs_listid: GlobalConfigItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfigItem {
    #[serde(flatten)]
    pub items: HashMap<String, GlobalConfigItemLang>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfigItemLang {
    pub title: Option<String>,
    pub count: u64,
    pub image: String,
    pub listid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesPlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub explicit_content: String,
    pub more_info: ModulesPlaylistMoreInfoRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesPlaylistMoreInfoRequest {
    pub uid: String,
    pub song_count: String,
    pub firstname: String,
    pub follower_count: String,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagMixRequest {
    pub explicit_content: String,
    pub id: String,
    pub image: String,
    pub language: String,
    pub list: String,
    pub list_count: String,
    pub list_type: String,
    pub perma_url: String,
    pub play_count: String,
    pub subtitle: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub year: String,
    pub more_info: TagMixRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagMixRequestMoreInfo {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromoRequest {
    pub id: String,
    pub image: String,
    pub perma_url: String,
    pub subtitle: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: Option<String>,
    pub list: Option<String>,
    pub list_count: Option<String>,
    pub list_type: Option<String>,
    pub play_count: Option<String>,
    pub year: Option<String>,
    pub explicit_content: String,
    pub more_info: PromoRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromoRequestMoreInfo {
    pub square_image: Option<String>,
    pub editorial_language: Option<String>,
    pub position: Option<String>,
    pub release_year: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleRequest {
    pub artist_recos: Option<ModuleItemRequest>,
    pub charts: ModuleItemRequest,
    pub city_mod: Option<ModuleItemRequest>,
    pub new_albums: ModuleItemRequest,
    pub new_trending: ModuleItemRequest,
    pub radio: ModuleItemRequest,
    pub tag_mixes: Option<ModuleItemRequest>,
    pub top_playlists: ModuleItemRequest,
    #[serde(flatten)]
    pub promos: HashMap<String, ModuleItemRequest>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ModuleItemRequest {
    pub title: String,
    pub subtitle: String,
    pub position: u64,
    pub featured_text: Option<String>,
}

/*---------------------- Response ---------------------- */

pub type RModules = Union<Value, CResponse<Union<Value, ModulesResponse>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesResponse {
    pub trending: Module<TrendingResponse>,
    pub albums: Module<Union<AlbumResponse, SongResponse>>,
    pub artist_recos: Module<ArtistRecoResponse>,
    pub charts: Module<ChartResponse>,
    pub city_mod: Module<CityModResponse>,
    pub discover: Module<DiscoverResonse>,
    pub mixes: Module<TagMixResponse>,
    pub playlists: Module<ModulesPlaylistResponse>,
    pub radio: Module<RadioResponse>,
    pub global_config: GlobalConfig,
    #[serde(flatten)]
    pub promos: HashMap<String, Module<PromoResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module<T> {
    pub title: String,
    pub subtitle: String,
    pub position: u64,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_text: Option<String>,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRecoResponse {
    pub id: String,
    pub image: Quality,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub subtitle: String,
    pub explicit: bool,
    pub featured_station_type: String,
    pub query: String,
    pub station_display_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityModResponse {
    pub id: String,
    pub image: Quality,
    pub url: String,
    pub subtitle: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub explicit: bool,
    pub query: Option<String>,
    pub album_id: Option<String>,
    pub featured_station_type: Option<String>,
    #[serde(default)]
    pub multiple_tunes: Option<Vec<CityModResponseMultipleTune>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityModResponseMultipleTune {
    pub id: String,
    pub subtype: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagMixResponse {
    pub id: String,
    pub image: Quality,
    pub language: String,
    pub list: String,
    pub list_count: u64,
    pub list_type: String,
    pub url: String,
    pub play_count: u64,
    pub subtitle: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub year: u64,
    pub explicit: bool,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesPlaylistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub user_id: String,
    pub song_count: u64,
    pub firstname: String,
    pub follower_count: u64,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverResonse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub badge: String,
    pub tags: Tags,
    pub sub_type: String,
    pub is_featured: bool,
    pub video_url: String,
    pub video_thumbnail: String,
    pub explicit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromoResponse {
    pub id: String,
    pub image: Quality,
    pub url: String,
    pub subtitle: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub explicit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editorial_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<u64>,
}
