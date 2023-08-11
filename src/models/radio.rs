use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::song::{SongRequest, SongResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct Radio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RadioError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioError {
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioSongRequest {
    pub stationid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub data: Option<HashMap<String, RadioSongDataRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioSongDataRequest {
    pub song: SongRequest,
}

/*---------------------- Response ---------------------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioSongResponse {
    pub stationid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub songs: Option<Vec<SongResponse>>,
}
