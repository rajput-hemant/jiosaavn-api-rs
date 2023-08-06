use crate::{
    models::album::{AlbumRequest, AlbumResponse, AlbumSongList},
    utils::{create_image_links, parse_explicit_content},
};

use super::song_payload;

/// Create album payload from album request
///
/// ## Arguments
///
/// * `album` - Album request
///
/// ## Returns
///
/// * `AlbumResponse` - Album payload
pub fn album_payload(album: AlbumRequest) -> AlbumResponse {
    AlbumResponse {
        id: album.id,
        name: album.title,
        subtitle: album.subtitle,
        type_field: album.type_field,
        image: create_image_links(&album.image),
        url: album.perma_url,
        explicit: parse_explicit_content(album.explicit_content),
        language: album.language,
        play_count: album.play_count.parse().unwrap_or_default(),
        year: album.year.parse().unwrap_or_default(),
        release_date: album.more_info.release_date,
        artist_map: album.more_info.artist_map,
        copyright_text: album.more_info.copyright_text,
        is_dolby_content: album.more_info.is_dolby_content,
        meta_html: album.more_info.meta_html,
        song_count: album
            .more_info
            .song_count
            .map(|f| f.parse().unwrap_or_default()),
        songs: match album.list {
            AlbumSongList::String(_) => vec![],
            AlbumSongList::List(list) => list.into_iter().map(song_payload).collect(),
        },
    }
}
