use crate::models::radio::{RadioSongRequest, RadioSongResponse};

use super::song_payload;

pub fn radio_song_payload(radio: RadioSongRequest) -> RadioSongResponse {
    RadioSongResponse {
        stationid: radio.stationid,
        error: radio.error,
        songs: match radio.data {
            Some(data) => Some(
                data.into_iter()
                    .map(|(_, song)| song_payload(song.song))
                    .collect(),
            ),
            None => None,
        },
    }
}
