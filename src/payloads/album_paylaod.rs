use crate::{
    models::{
        album::{AlbumRequest, AlbumResponse},
        artist::ArtistMapResponse,
        misc::Union,
    },
    utils::{create_image_links, parse_explicit_content, parse_type},
};

use super::{artist_payload::artist_map_payload, song_payload};

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
        image: create_image_links(album.image),
        url: album.perma_url,
        explicit: parse_explicit_content(album.explicit_content),
        language: album.language,
        play_count: parse_type(album.play_count),
        year: parse_type(album.year),
        release_date: album.more_info.release_date,
        artist_map: album
            .more_info
            .artist_map
            .map(|artist_map| match artist_map {
                Union::B(map) => artist_map_payload(map),
                _ => ArtistMapResponse::default(),
            }),
        copyright_text: album.more_info.copyright_text,
        is_dolby_content: album.more_info.is_dolby_content,
        song_count: album.more_info.song_count.map(parse_type),
        songs: match album.list {
            Union::A(_) => vec![],
            Union::B(list) => list.into_iter().map(song_payload).collect(),
        },
    }
}
