use super::{artist_payload::artist_mini_payload, song_payload};
use crate::{
    models::{
        misc::Union,
        playlist::{
            PlaylistModulesArtistsResponse, PlaylistModulesCurrentlyTrendingPlaylistsResponse,
            PlaylistModulesRelatedParamsResponse, PlaylistModulesRelatedResponse,
            PlaylistModulesRequest, PlaylistModulesResponse,
            PlaylistModulesTrendingPlaylistsParamsResponse, PlaylistRecRequest,
            PlaylistRecResponse, PlaylistRequest, PlaylistResponse,
        },
    },
    utils::{create_image_links, parse_bool, parse_type},
};

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
            follower_count: more_info.follower_count.map(parse_type),
            user_id: more_info.uid,
            header_desc: playlist.header_desc,
            last_updated: more_info.last_updated.map(parse_type),
            username: more_info.username,
            firstname: more_info.firstname,
            lastname: more_info.lastname,
            image: create_image_links(playlist.image),
            share: more_info.share.map(parse_type),
            type_field: playlist.type_field,
            list_count: playlist.list_count.map(parse_type),
            is_dolby_content: more_info.is_dolby_content,
            video_count: more_info.video_count.map(parse_type),
            fan_count: more_info.fan_count.map(|i| parse_type(i.replace(',', ""))),
            artists: more_info
                .artists
                .map(|a| a.into_iter().map(artist_mini_payload).collect()),
            explicit: parse_bool(playlist.explicit_content),
            language: playlist.language,
            play_count: playlist.play_count.map(parse_type),
            list_type: more_info.playlist_type,
            subtitle: playlist.subtitle,
            subtitle_desc: more_info.subtitle_desc,
            year: playlist.year.map(parse_type),
            songs: match playlist.list {
                Some(songs) => match songs {
                    Union::A(_) => Some(vec![]),
                    Union::B(list) => Some(list.into_iter().map(song_payload).collect()),
                },
                None => Some(vec![]),
            },
            modules: playlist.modules.map(playlist_modules_payload),
        }
    }
}

fn playlist_modules_payload(modules: PlaylistModulesRequest) -> PlaylistModulesResponse {
    PlaylistModulesResponse {
        related_playlist: PlaylistModulesRelatedResponse {
            title: modules.related_playlist.title,
            subtitle: modules.related_playlist.subtitle,
            source: "/playlist/recommend".to_string(),
            position: modules.related_playlist.position,
            params: PlaylistModulesRelatedParamsResponse {
                id: modules.related_playlist.source_params.listid,
            },
        },

        currently_trending_playlists: PlaylistModulesCurrentlyTrendingPlaylistsResponse {
            title: modules.currently_trending_playlists.title,
            subtitle: modules.currently_trending_playlists.subtitle,
            source: "/get/trending".to_string(),
            position: modules.currently_trending_playlists.position,
            params: PlaylistModulesTrendingPlaylistsParamsResponse {
                type_field: modules
                    .currently_trending_playlists
                    .source_params
                    .entity_type,
                lang: modules
                    .currently_trending_playlists
                    .source_params
                    .entity_language,
            },
        },

        artists: PlaylistModulesArtistsResponse {
            title: modules.artists.title,
            subtitle: modules.artists.subtitle,
            source: modules.artists.source,
            position: modules.artists.position,
        },
    }
}

pub fn playlist_recommend_payload(playlist: PlaylistRecRequest) -> PlaylistRecResponse {
    PlaylistRecResponse {
        id: playlist.id,
        name: playlist.title,
        subtitle: playlist.subtitle,
        type_field: playlist.type_field,
        url: playlist.perma_url,
        image: create_image_links(playlist.image),
        explicit: parse_bool(playlist.explicit_content),
        firstname: playlist.more_info.firstname,
    }
}
