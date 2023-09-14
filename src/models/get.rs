use super::{
    album::{AlbumRequest, AlbumResponse},
    misc::Union3,
    playlist::{PlaylistRequest, PlaylistResponse},
    misc::Quality,
    song::{SongRequest, SongResponse},
};
use serde::{Deserialize, Serialize};

pub struct A<T> {
    pub count: i64,
    pub last_page: bool,
    pub data: Vec<T>,
}

pub type TrendingRequest = Vec<Union3<AlbumRequest, SongRequest, PlaylistRequest>>;

pub type FeaturedPlaylistsRequest = A<PlaylistRequest>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartRequest {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: String,
    pub perma_url: String,
    pub count: Option<u64>,
    pub listname: Option<String>,
    pub language: Option<String>,
    pub explicit_content: Option<String>,
    pub more_info: Option<ChartRequestMoreInfo>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ChartRequestMoreInfo {
    pub firstname: Option<String>,
    pub song_count: Option<u64>,
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
    pub explicit_content: String,
    pub more_info: RadioRequestMoreInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioRequestMoreInfo {
    pub description: Option<String>,
    pub featured_station_type: String,
    pub query: Option<String>,
    pub color: Option<String>,
    pub language: String,
    pub station_display_text: String,
}

/*---------------------- Response ---------------------- */

pub type TrendingResponse = Union3<AlbumResponse, SongResponse, PlaylistResponse>;

pub type FeaturedPlaylistsResponse = A<PlaylistResponse>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartResponse {
    pub id: String,
    pub image: Quality,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioResponse {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image: Quality,
    pub url: String,
    pub explicit: bool,
    pub description: Option<String>,
    pub featured_station_type: String,
    pub query: Option<String>,
    pub color: Option<String>,
    pub language: String,
    pub station_display_text: String,
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
