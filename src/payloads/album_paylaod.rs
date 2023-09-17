use crate::{
    models::{
        album::{
            AlbumModulesArtistsResponse, AlbumModulesCurrentlyTrendingResponse,
            AlbumModulesRecommendResponse, AlbumModulesRequest, AlbumModulesResponse,
            AlbumModulesTopAlbumsFromSameYearResponse, AlbumRecommendParamsResponse, AlbumRequest,
            AlbumResponse, AlbumTrendingParamsResponse, AlbumYearLangParamsResponse,
        },
        misc::Union,
    },
    utils::{create_image_links, parse_bool, parse_type},
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
        header_desc: album.header_desc,
        explicit: parse_bool(album.explicit_content),
        language: album.language,
        play_count: parse_type(album.play_count),
        year: parse_type(album.year),
        label_url: album.more_info.label_url,
        list_count: parse_type(album.list_count),
        list_type: album.list_type,
        artist_map: album.more_info.artist_map.map(artist_map_payload),
        copyright_text: album.more_info.copyright_text,
        is_dolby_content: album.more_info.is_dolby_content,
        song_count: album.more_info.song_count.map(parse_type),
        songs: match album.list {
            Some(songs) => match songs {
                Union::A(_) => Some(vec![]),
                Union::B(list) => Some(list.into_iter().map(song_payload).collect()),
            },
            None => Some(vec![]),
        },
        // songs: album.list.map_or_else(
        //     || Some(vec![]),
        //     |songs| match songs {
        //         Union::A(_) => Some(vec![]),
        //         Union::B(list) => Some(list.into_iter().map(song_payload).collect()),
        //     },
        // ),
        modules: album.modules.map(album_module_payload),
    }
}

/// Create payload for multiple albums
/// 
/// ## Arguments
/// 
/// * `albums` - Vector of album requests
/// 
/// ## Returns
/// 
/// * `Vec<AlbumResponse>` - Vector of album payloads
pub fn albums_payload(albums: Vec<AlbumRequest>) -> Vec<AlbumResponse> {
    albums.into_iter().map(album_payload).collect()
}

fn album_module_payload(module: AlbumModulesRequest) -> AlbumModulesResponse {
    AlbumModulesResponse {
        recommend: AlbumModulesRecommendResponse {
            title: module.reco.title,
            subtitle: module.reco.subtitle,
            source: "/album/recommend".to_string(),
            position: module.reco.position,
            params: AlbumRecommendParamsResponse {
                id: module.reco.source_params.albumid,
            },
        },

        currently_trending: AlbumModulesCurrentlyTrendingResponse {
            title: module.currently_trending.title,
            subtitle: module.currently_trending.subtitle,
            source: "/get/trending".to_string(),
            position: module.currently_trending.position,
            params: AlbumTrendingParamsResponse {
                type_field: module.currently_trending.source_params.entity_type,
                lang: module.currently_trending.source_params.entity_language,
            },
        },

        top_albums_from_same_year: AlbumModulesTopAlbumsFromSameYearResponse {
            title: module.top_albums_from_same_year.title,
            subtitle: module.top_albums_from_same_year.subtitle,
            source: "/album/same-year".to_string(),
            position: module.currently_trending.position,
            params: AlbumYearLangParamsResponse {
                lang: module.top_albums_from_same_year.source_params.album_lang,
                year: module.top_albums_from_same_year.source_params.album_year,
            },
        },

        artists: AlbumModulesArtistsResponse {
            title: module.artists.title,
            subtitle: module.artists.subtitle,
            source: module.artists.source,
            position: module.artists.position,
        },
    }
}
