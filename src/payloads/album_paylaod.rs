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

use super::{artist_payload::artist_map_payload, songs_payload};

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
    let a = album;
    let m = a.more_info;

    AlbumResponse {
        id: a.id,
        name: a.title,
        subtitle: a.subtitle,
        type_field: a.type_field,
        image: create_image_links(a.image),
        url: a.perma_url,
        header_desc: a.header_desc,
        explicit: parse_bool(a.explicit_content),
        language: a.language,
        play_count: parse_type(a.play_count),
        year: parse_type(a.year),
        label_url: m.label_url,
        list_count: parse_type(a.list_count),
        list_type: a.list_type,
        artist_map: m.artist_map.map(artist_map_payload),
        copyright_text: m.copyright_text,
        is_dolby_content: m.is_dolby_content,
        song_count: m.song_count.map(parse_type),
        songs: Some(a.list.map_or(vec![], |songs| match songs {
            Union::A(_) => vec![],
            Union::B(songs) => songs_payload(songs),
        })),
        modules: a.modules.map(album_module_payload),
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
    let m = module;
    let (r, c, t, a) = (
        m.reco,
        m.currently_trending,
        m.top_albums_from_same_year,
        m.artists,
    );

    AlbumModulesResponse {
        recommend: AlbumModulesRecommendResponse {
            title: r.title,
            subtitle: r.subtitle,
            source: "/album/recommend".to_string(),
            position: r.position,
            params: AlbumRecommendParamsResponse {
                id: r.source_params.albumid,
            },
        },

        currently_trending: AlbumModulesCurrentlyTrendingResponse {
            title: c.title,
            subtitle: c.subtitle,
            source: "/get/trending".to_string(),
            position: c.position,
            params: AlbumTrendingParamsResponse {
                type_field: c.source_params.entity_type,
                lang: c.source_params.entity_language,
            },
        },

        top_albums_from_same_year: AlbumModulesTopAlbumsFromSameYearResponse {
            title: t.title,
            subtitle: t.subtitle,
            source: "/album/same-year".to_string(),
            position: t.position,
            params: AlbumYearLangParamsResponse {
                lang: t.source_params.album_lang,
                year: t.source_params.album_year,
            },
        },

        artists: AlbumModulesArtistsResponse {
            title: a.title,
            subtitle: a.subtitle,
            source: a.source,
            position: a.position,
        },
    }
}
