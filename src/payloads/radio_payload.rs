use crate::models::radio::{
    RadioSongRequest, RadioSongResponse, RadioStationRequest, RadioStationResponse,
};

use super::song_payload;

/// Create radio station payload from radio station request
///
/// ## Arguments
///
/// * `station` - Radio station request
///
/// ## Returns
///
/// * `RadioStationResponse` - Radio station payload
pub fn radio_station_payload(station: RadioStationRequest) -> RadioStationResponse {
    RadioStationResponse {
        station_id: station.stationid,
    }
}

/// Create radio songs payload from radio songs request
///
/// ## Arguments
///
/// * `radio` - Radio songs request
///
/// ## Returns
///
/// * `RadioSongResponse` - Radio songs payload
pub fn radio_songs_payload(radio: RadioSongRequest) -> RadioSongResponse {
    let station_id = radio.stationid;
    let mut songs = Vec::new();

    if let Some(song) = radio.song {
        songs.push(song_payload(song));
        return RadioSongResponse { station_id, songs };
    }

    if let Some(song) = radio.data {
        for (k, v) in song {
            if k == "stationid" || k == "error" {
                continue;
            }
            songs.push(song_payload(v.song));
        }
    }

    RadioSongResponse { station_id, songs }
}
