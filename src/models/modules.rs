use serde::{Deserialize, Serialize};

use super::{
    album::{AlbumRequest, AlbumResponse},
    artist::{ArtistMapRequest, ArtistMapResponse},
    misc::Rights,
    quality::Quality,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesRequest {
    pub artist_recos: Vec<ArtistRecoRequest>,
    pub browse_discover: Vec<DiscoverRequest>,
    pub charts: Vec<ChartRequest>,
    pub city_mod: Vec<CityModRequest>,
    pub global_config: GlobalConfig,
    pub new_albums: Vec<AlbumRequest>,
    pub new_trending: Vec<TrendingRequest>,
    pub top_playlists: Vec<ModulePlaylistRequest>,
    pub tag_mixes: Vec<TagMixRequest>,
    pub radio: Vec<RadioRequest>,
    pub modules: ModuleRequest,
    // pub top_shows: TopShowsRequest,
    #[serde(rename = "promo:vx:data:107")]
    pub promo_107: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:112")]
    pub promo_112: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:113")]
    pub promo_113: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:114")]
    pub promo_114: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:116")]
    pub promo_116: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:118")]
    pub promo_118: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:176")]
    pub promo_176: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:185")]
    pub promo_185: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:49")]
    pub promo_49: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:68")]
    pub promo_68: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:76")]
    pub promo_76: Vec<PromoRequest>,
    #[serde(rename = "promo:vx:data:90")]
    pub promo_90: Vec<PromoRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRecoRequest {
    pub id: String,
    pub image: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
    pub subtitle: String,
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
pub struct ChartRequest {
    pub id: String,
    pub image: String,
    pub title: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub count: Option<u64>,
    pub listname: Option<String>,
    pub language: Option<String>,
    pub subtitle: Option<String>,
    pub explicit_content: Option<String>,
    pub more_info: Option<ChartRequestMoreInfo>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ChartRequestMoreInfo {
    pub firstname: Option<String>,
    pub song_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityModRequest {
    pub id: String,
    pub image: String,
    pub perma_url: String,
    pub subtitle: String,
    pub title: String,
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
    pub subtype: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub vcode: String,
    pub vlink: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub random_songs_listid: GlobalConfigItem,
    pub weekly_top_songs_listid: GlobalConfigItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfigItem {
    pub english: GlobalConfigItemLang,
    pub hindi: GlobalConfigItemLang,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfigItemLang {
    pub title: Option<String>,
    pub count: u64,
    pub image: String,
    pub listid: String,
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
    pub more_info: TagMixRequestMoreInfo,
    pub perma_url: String,
    pub play_count: String,
    pub subtitle: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub year: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagMixRequestMoreInfo {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: RadioRequestMoreInfo,
    pub explicit_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioRequestMoreInfo {
    pub description: String,
    pub featured_station_type: String,
    pub query: String,
    pub color: String,
    pub language: String,
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
    pub sub_type: String,
    pub available: String,
    pub is_featured: String,
    pub video_url: String,
    pub video_thumbnail: String,
}

// #[derive(Debug, Deserialize)]
// pub struct TopShowsRequest {
//     pub shows: Vec<ShowRequest>,
//     pub last_page: bool,
// }

// #[derive(Debug, Deserialize)]
// pub struct ShowRequest {
//     pub id: String,
//     pub title: String,
//     pub subtitle: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub image: String,
//     pub perma_url: String,
//     pub more_info: ShowMoreInfo,
//     pub explicit_content: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct ShowMoreInfo {
//     pub season_number: String,
//     pub release_date: String,
//     pub year: String,
//     pub badge: String,
//     pub square_image: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
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
    pub more_info: TrendingMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingMoreInfo {
    pub release_date: Option<String>,
    pub song_count: Option<String>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMapRequest>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulePlaylistRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub more_info: PlaylistMoreInfo,
    pub explicit_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistMoreInfo {
    pub song_count: String,
    pub firstname: String,
    pub follower_count: String,
    pub last_updated: String,
    pub uid: String,
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
    pub radio: ModuleItemRequest,
    pub charts: ModuleItemRequest,
    pub city_mod: ModuleItemRequest,
    pub tag_mixes: ModuleItemRequest,
    pub new_albums: ModuleItemRequest,
    pub new_trending: ModuleItemRequest,
    pub artist_recos: ModuleItemRequest,
    pub top_playlists: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:107")]
    pub promo_107: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:112")]
    pub promo_112: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:113")]
    pub promo_113: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:114")]
    pub promo_114: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:116")]
    pub promo_116: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:118")]
    pub promo_118: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:176")]
    pub promo_176: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:185")]
    pub promo_185: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:49")]
    pub promo_49: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:68")]
    pub promo_68: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:76")]
    pub promo_76: ModuleItemRequest,
    #[serde(rename = "promo:vx:data:90")]
    pub promo_90: ModuleItemRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleItemRequest {
    pub title: String,
    pub subtitle: String,
    pub featured_text: Option<String>,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Deserialize, Serialize)]
pub struct Module<T> {
    pub title: String,
    pub subtitle: String,
    pub featured_text: Option<String>,
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModulesResponse {
    pub artist_recos: Module<Vec<ArtistRecoResponse>>,
    pub discover: Vec<DiscoverResonse>,
    pub charts: Module<Vec<ChartResponse>>,
    pub city_mod: Module<Vec<CityModResponse>>,
    pub global_config: GlobalConfig,
    pub albums: Module<Vec<AlbumResponse>>,
    pub trending: Module<Vec<TrendingResponse>>,
    pub playlists: Module<Vec<ModulePlaylistResponse>>,
    pub mixes: Module<Vec<TagMixResponse>>,
    pub radio: Module<Vec<RadioResponse>>,
    pub promo_107: Module<Vec<PromoResponse>>,
    pub promo_112: Module<Vec<PromoResponse>>,
    pub promo_113: Module<Vec<PromoResponse>>,
    pub promo_114: Module<Vec<PromoResponse>>,
    pub promo_116: Module<Vec<PromoResponse>>,
    pub promo_118: Module<Vec<PromoResponse>>,
    pub promo_176: Module<Vec<PromoResponse>>,
    pub promo_185: Module<Vec<PromoResponse>>,
    pub promo_49: Module<Vec<PromoResponse>>,
    pub promo_68: Module<Vec<PromoResponse>>,
    pub promo_76: Module<Vec<PromoResponse>>,
    pub promo_90: Module<Vec<PromoResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRecoResponse {
    pub id: String,
    pub image: Quality,
    pub name: String,
    pub query: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub subtitle: String,
    pub explicit: bool,
    pub featured_station_type: String,
    pub station_display_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartResponse {
    pub id: String,
    pub image: Quality,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub count: Option<u64>,
    pub listname: Option<String>,
    pub language: Option<String>,
    pub subtitle: Option<String>,
    pub explicit: Option<bool>,
    pub firstname: Option<String>,
    pub song_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct RadioResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: String,
    pub description: String,
    pub featured_station_type: String,
    pub query: String,
    pub color: String,
    pub language: String,
    pub station_display_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverResonse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub badge: String,
    pub sub_type: String,
    pub is_featured: bool,
    pub video_url: String,
    pub video_thumbnail: String,
    pub explicit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub field: String,
    pub url: String,
    pub image: Quality,
    pub language: String,
    pub year: u64,
    pub play_count: u64,
    pub explicit: bool,
    pub list_count: u64,
    pub list_type: String,
    pub list: String,
    pub release_date: Option<String>,
    pub song_count: Option<u64>,
    #[serde(rename = "artistMap")]
    pub artist_map: Option<ArtistMapResponse>,
    pub music: Option<String>,
    pub album_id: Option<String>,
    pub album: Option<String>,
    pub label: Option<String>,
    pub origin: Option<String>,
    pub is_dolby_content: Option<bool>,
    #[serde(rename = "320kbps")]
    pub _320kbps: Option<bool>,
    pub encrypted_media_url: Option<String>,
    pub encrypted_cache_url: Option<String>,
    pub album_url: Option<String>,
    pub duration: Option<u64>,
    pub rights: Option<Rights>,
    pub cache_state: Option<String>,
    pub has_lyrics: Option<String>,
    pub lyrics_snippet: Option<String>,
    pub copyright_text: Option<String>,
    pub label_url: Option<String>,
    pub lyrics_id: Option<String>,
    #[serde(rename = "isWeekly")]
    pub firstname: Option<String>,
    pub follower_count: Option<u64>,
    pub fan_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulePlaylistResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub song_count: u64,
    pub follower_count: u64,
    pub last_updated: u64,
    pub firstname: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoResponse {
    pub id: String,
    pub image: Quality,
    pub url: String,
    pub subtitle: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: Option<String>,
    pub list: Option<String>,
    pub list_count: Option<u64>,
    pub list_type: Option<String>,
    pub play_count: Option<u64>,
    pub year: Option<u64>,
    pub explicit: bool,
    pub square_image: Option<String>,
    pub editorial_language: Option<String>,
    pub position: Option<u64>,
    pub release_year: Option<u64>,
}
