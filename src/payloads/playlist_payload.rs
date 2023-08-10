use crate::{
    models::playlist::{PlaylistRequest, PlaylistResponse},
    utils::{create_image_links, parse_explicit_content, parse_type},
};

use super::{artist_payload::artist_mini_payload, song_payload};

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
    {
        let more_info = playlist.more_info;

        PlaylistResponse {
            id: playlist.id,
            name: playlist.title,
            url: playlist.perma_url,
            follower_count: parse_type(more_info.follower_count),
            user_id: more_info.uid,
            last_updated: parse_type(more_info.last_updated),
            username: more_info.username,
            firstname: more_info.firstname,
            lastname: more_info.lastname,
            image: create_image_links(playlist.image),
            share: parse_type(more_info.share),
            type_field: playlist.type_field,
            list_count: parse_type(playlist.list_count),
            fan_count: parse_type(more_info.fan_count.replace(",", "")),
            is_dolby_content: more_info.is_dolby_content,
            video_count: parse_type(more_info.video_count),
            artists: more_info
                .artists
                .into_iter()
                .map(artist_mini_payload)
                .collect(),
            explicit_content: parse_explicit_content(playlist.explicit_content),
            language: playlist.language,
            play_count: parse_type(playlist.play_count),
            list_type: more_info.playlist_type,
            subtitle: playlist.subtitle,
            subtitle_desc: more_info.subtitle_desc,
            year: parse_type(playlist.year),
            songs: playlist.list.into_iter().map(song_payload).collect(),
        }
    }
}
