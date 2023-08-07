use crate::{
    models::song::{SongRequest, SongResponse},
    utils::create_image_links,
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
    SongResponse {
        id: song.id,
        name: song.title,
        subtitle: song.subtitle,
        type_field: song.type_field,
        url: song.perma_url,
        image: create_image_links(&song.image),
        language: song.language,
        year: song.year.parse().unwrap_or_default(),
        play_count: song.play_count.parse().unwrap_or_default(),
        explicit_content: song.explicit_content.parse().unwrap_or_default(),
        list_count: song.list_count.parse().unwrap_or_default(),
        list_type: song.list_type,
        list: song.list,
        music: song.more_info.music,
        album_id: song.more_info.album_id,
        album: song.more_info.album,
        label: song.more_info.label,
        song: song.more_info.song,
        origin: song.more_info.origin,
        is_dolby_content: song.more_info.is_dolby_content,
        _320kbps: song.more_info._320kbps.parse().unwrap_or_default(),
        encrypted_media_url: song.more_info.encrypted_media_url,
        encrypted_cache_url: song.more_info.encrypted_cache_url,
        album_url: song.more_info.album_url,
        duration: song.more_info.duration.parse().unwrap_or_default(),
        rights: song.more_info.rights,
        cache_state: song.more_info.cache_state,
        has_lyrics: song.more_info.has_lyrics.parse().unwrap_or_default(),
        lyrics_snippet: song.more_info.lyrics_snippet,
        starred: song.more_info.starred.parse().unwrap_or_default(),
        artist_map: artist_map_payload(song.more_info.artist_map),
        release_date: song.more_info.release_date.unwrap_or_default(),
        vcode: song.more_info.vcode,
        vlink: song.more_info.vlink,
        triller_available: song.more_info.triller_available,
        copyright_text: song.more_info.copyright_text,
        lyrics_id: song.more_info.lyrics_id,
    }
}
