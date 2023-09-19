use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    misc::Union,
    response::CResponse,
    song::{SongRequest, SongResponse},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioStationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RadioError>,
    pub stationid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioError {
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioSongRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub stationid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song: Option<SongRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub data: Option<HashMap<String, RadioSongDataRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioSongDataRequest {
    pub song: SongRequest,
}

/*---------------------- Response ---------------------- */
pub type RRadioStation = Union<Value, CResponse<Union<Value, RadioStationResponse>>>;
pub type RRadioSongs = Union<Value, CResponse<Union<Value, RadioSongResponse>>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RadioStationType {
    Featured,
    Artist,
    Entity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioStationResponse {
    pub station_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RadioSongResponse {
    pub station_id: String,
    pub songs: Vec<SongResponse>,
}
