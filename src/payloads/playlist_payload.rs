use crate::{
    models::playlist::{PlaylistRequest, PlaylistResponse},
    utils::{create_image_links, parse_explicit_content},
};

use super::song_payload;

/// Create playlist payload from playlist request
///
/// ## Arguments
///
/// * `playlist` - Playlist request
///
/// ## Returns
///
/// * `PlaylistResponse` - Playlist payload
pub fn playlist_payload(playlist: PlaylistRequest) -> PlaylistResponse {
    PlaylistResponse {
        id: playlist.id,
        name: playlist.title,
        url: playlist.perma_url,
        follower_count: playlist
            .more_info
            .follower_count
            .parse()
            .unwrap_or_default(),
        user_id: playlist.more_info.uid,
        last_updated: playlist.more_info.last_updated.parse().unwrap_or_default(),
        username: playlist.more_info.username,
        firstname: playlist.more_info.firstname,
        lastname: playlist.more_info.lastname,
        image: create_image_links(&playlist.image),
        share: playlist.more_info.share.parse().unwrap_or_default(),
        type_field: playlist.type_field,
        list_count: playlist.list_count.parse().unwrap_or_default(),
        fan_count: playlist
            .more_info
            .fan_count
            .replace(",", "")
            .parse()
            .unwrap_or_default(),
        is_dolby_content: playlist.more_info.is_dolby_content,
        video_count: playlist.more_info.video_count.parse().unwrap_or_default(),
        artists: playlist.more_info.artists,
        explicit_content: parse_explicit_content(playlist.explicit_content),
        language: playlist.language,
        play_count: playlist.play_count.parse().unwrap_or_default(),
        list_type: playlist.more_info.playlist_type,
        subtitle: playlist.subtitle,
        subtitle_desc: playlist.more_info.subtitle_desc,
        year: playlist.year.parse().unwrap_or_default(),
        songs: playlist.list.into_iter().map(song_payload).collect(),
    }
}
