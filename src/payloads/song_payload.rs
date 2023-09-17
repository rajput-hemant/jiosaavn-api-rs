use super::artist_payload::artist_map_payload;
use crate::{
    models::{
        misc::Union,
        song::{
            SongModulesActorSongParamsResponse, SongModulesArtistSongParamsResponse,
            SongModulesArtistsResponse, SongModulesCurrentlyTrendingResponse,
            SongModulesRecommendParamsResponse, SongModulesRecommendResponse, SongModulesRequest,
            SongModulesResponse, SongModulesSongsBySameActorsResponse,
            SongModulesSongsBySameArtistsResponse, SongModulesTrendingParamsResponse,
            SongObjRequest, SongObjResponse, SongRequest, SongResponse,
        },
    },
    utils::{create_download_links, create_image_links, parse_bool, parse_type},
};

pub fn song_obj_payload(obj: SongObjRequest) -> SongObjResponse {
    SongObjResponse {
        songs: obj.songs.into_iter().map(song_payload).collect(),
        modules: obj.modules.map(song_modules_payload),
    }
}

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
    let s = song;
    let m = s.more_info;

    SongResponse {
        id: s.id,
        name: s.title,
        subtitle: s.subtitle,
        type_field: s.type_field,
        url: s.perma_url,
        image: create_image_links(s.image),
        language: s.language,
        year: parse_type(s.year),
        header_desc: s.header_desc,
        play_count: parse_type(match s.play_count {
            Union::A(play_count) => play_count,
            Union::B(play_count) => play_count.to_string(),
        }),
        explicit: parse_bool(s.explicit_content),
        list_count: parse_type(s.list_count),
        list_type: s.list_type,
        list: s.list,
        music: m.music,
        album_id: m.album_id,
        album: m.album,
        label: m.label,
        label_url: m.label_url,
        origin: m.origin,
        is_dolby_content: m.is_dolby_content,
        _320kbps: parse_bool(m._320kbps),
        download_url: create_download_links(m.encrypted_media_url),
        album_url: m.album_url,
        duration: parse_type(m.duration),
        rights: m.rights,
        has_lyrics: parse_bool(m.has_lyrics),
        lyrics_id: m.lyrics_id,
        lyrics_snippet: m.lyrics_snippet,
        starred: parse_bool(m.starred),
        artist_map: artist_map_payload(m.artist_map),
        release_date: m.release_date,
        triller_available: m.triller_available,
        vcode: m.vcode,
        vlink: m.vlink,
        copyright_text: m.copyright_text,
    }
}

/// Create payload for multiple songs
///
/// ## Arguments
///
/// * `songs` - Vector of song requests
///
/// ## Returns
///
/// * `Vec<SongResponse>` - Vector of song payloads
pub fn songs_payload(songs: Vec<SongRequest>) -> Vec<SongResponse> {
    songs.into_iter().map(song_payload).collect()
}

fn song_modules_payload(modules: SongModulesRequest) -> SongModulesResponse {
    let m = modules;
    let (r, c, s_ar, s_ac, a) = (
        m.reco,
        m.currently_trending,
        m.songs_by_same_artists,
        m.songs_by_same_actors,
        m.artists,
    );

    SongModulesResponse {
        recommend: SongModulesRecommendResponse {
            title: r.title,
            subtitle: r.subtitle,
            source: "/song/recommend".to_string(),
            position: r.position,
            params: SongModulesRecommendParamsResponse {
                id: r.source_params.pid,
                lang: r.source_params.language,
            },
        },

        currently_trending: SongModulesCurrentlyTrendingResponse {
            title: c.title,
            subtitle: c.subtitle,
            source: "/get/trending".to_string(),
            position: c.position,
            params: SongModulesTrendingParamsResponse {
                type_field: c.source_params.entity_type,
                lang: c.source_params.entity_language,
            },
        },

        songs_by_same_artists: SongModulesSongsBySameArtistsResponse {
            title: s_ar.title,
            subtitle: s_ar.subtitle,
            source: "/artist/top-songs".to_string(),
            position: s_ar.position,
            params: SongModulesArtistSongParamsResponse {
                artist_id: s_ar.source_params.artist_ids,
                song_id: s_ar.source_params.song_id,
                lang: s_ar.source_params.language,
            },
        },

        songs_by_same_actors: SongModulesSongsBySameActorsResponse {
            title: s_ac.title,
            subtitle: s_ac.subtitle,
            source: "/get/actor-top-songs".to_string(),
            position: s_ac.position,
            params: SongModulesActorSongParamsResponse {
                actor_id: s_ac.source_params.actor_ids,
                song_id: s_ac.source_params.song_id,
                lang: s_ac.source_params.language,
            },
        },

        artists: SongModulesArtistsResponse {
            title: a.title,
            subtitle: a.subtitle,
            source: a.source,
            position: a.position,
        },
    }
}
