use crate::{
    models::song::{SongRequest, SongResponse},
    utils::{create_download_links, create_image_links, parse_type},
};

use super::artist_payload::artist_map_payload;
/// Create song payload from song request
///
/// ## Arguments
///
/// * `song` - Song request
///
/// ## Returns
///
/// * `SongResponse` - Song payload
pub fn song_payload(song: SongRequest) -> SongResponse {
    {
        let more_info = song.more_info;

        SongResponse {
            id: song.id,
            name: song.title,
            subtitle: song.subtitle,
            type_field: song.type_field,
            url: song.perma_url,
            image: create_image_links(song.image),
            language: song.language,
            year: parse_type(song.year),
            play_count: parse_type(song.play_count),
            explicit_content: parse_type(song.explicit_content),
            list_count: parse_type(song.list_count),
            list_type: song.list_type,
            list: song.list,
            music: more_info.music,
            album_id: more_info.album_id,
            album: more_info.album,
            label: more_info.label,
            song: more_info.song,
            origin: more_info.origin,
            is_dolby_content: more_info.is_dolby_content,
            _320kbps: parse_type(more_info._320kbps),
            download_url: create_download_links(more_info.encrypted_media_url),
            album_url: more_info.album_url,
            duration: parse_type(more_info.duration),
            rights: more_info.rights,
            cache_state: more_info.cache_state,
            has_lyrics: parse_type(more_info.has_lyrics),
            lyrics_snippet: more_info.lyrics_snippet,
            starred: parse_type(more_info.starred),
            artist_map: artist_map_payload(more_info.artist_map),
            release_date: more_info.release_date.unwrap_or_default(),
            triller_available: more_info.triller_available,
            copyright_text: more_info.copyright_text,
            lyrics_id: more_info.lyrics_id,
        }
    }
}
