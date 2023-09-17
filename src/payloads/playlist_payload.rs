use super::{artist_payload::artist_mini_payload, songs_payload};
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
        let p = playlist;
        let m = p.more_info;

        PlaylistResponse {
            id: p.id,
            name: p.title,
            url: p.perma_url,
            follower_count: m.follower_count.map(parse_type),
            user_id: m.uid,
            header_desc: p.header_desc,
            last_updated: m.last_updated.map(parse_type),
            username: m.username,
            firstname: m.firstname,
            lastname: m.lastname,
            image: create_image_links(p.image),
            share: m.share.map(parse_type),
            type_field: p.type_field,
            list_count: p.list_count.map(parse_type),
            is_dolby_content: m.is_dolby_content,
            video_count: m.video_count.map(parse_type),
            fan_count: m.fan_count.map(|i| parse_type(i.replace(',', ""))),
            artists: m.artists.map(artist_mini_payload),
            explicit: parse_bool(p.explicit_content),
            language: p.language,
            play_count: p.play_count.map(parse_type),
            list_type: m.playlist_type,
            subtitle: p.subtitle,
            subtitle_desc: m.subtitle_desc,
            year: p.year.map(parse_type),
            songs: Some(p.list.map_or(vec![], |songs| match songs {
                Union::A(_) => vec![],
                Union::B(songs) => songs_payload(songs),
            })),
            modules: p.modules.map(playlist_modules_payload),
        }
    }
}

/// Create playlist recommend payload from playlist recommend request
///
/// ## Arguments
///
/// * `playlists` - Playlist recommend request
///
/// ## Returns
///
/// * `Vec<PlaylistRecResponse>` - Playlist recommend payload
pub fn playlist_recommend_payload(playlists: Vec<PlaylistRecRequest>) -> Vec<PlaylistRecResponse> {
    playlists
        .into_iter()
        .map(|p| PlaylistRecResponse {
            id: p.id,
            name: p.title,
            subtitle: p.subtitle,
            type_field: p.type_field,
            url: p.perma_url,
            image: create_image_links(p.image),
            explicit: parse_bool(p.explicit_content),
            firstname: p.more_info.firstname,
        })
        .collect()
}

fn playlist_modules_payload(modules: PlaylistModulesRequest) -> PlaylistModulesResponse {
    let (r, c, a) = (
        modules.related_playlist,
        modules.currently_trending_playlists,
        modules.artists,
    );

    PlaylistModulesResponse {
        related_playlist: PlaylistModulesRelatedResponse {
            title: r.title,
            subtitle: r.subtitle,
            source: "/playlist/recommend".to_string(),
            position: r.position,
            params: PlaylistModulesRelatedParamsResponse {
                id: r.source_params.listid,
            },
        },

        currently_trending_playlists: PlaylistModulesCurrentlyTrendingPlaylistsResponse {
            title: c.title,
            subtitle: c.subtitle,
            source: "/get/trending".to_string(),
            position: c.position,
            params: PlaylistModulesTrendingPlaylistsParamsResponse {
                type_field: c.source_params.entity_type,
                lang: c.source_params.entity_language,
            },
        },

        artists: PlaylistModulesArtistsResponse {
            title: a.title,
            subtitle: a.subtitle,
            source: a.source,
            position: a.position,
        },
    }
}
