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
        modules: match obj.modules {
            Some(modules) => Some(song_modules_payload(modules)),
            None => None,
        },
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
    let more_info = song.more_info;

    SongResponse {
        id: song.id,
        name: song.title,
        subtitle: song.subtitle,
        type_field: song.type_field,
        url: song.perma_url,
        image: create_image_links(song.image),
        language: song.language,
        year: parse_type(song.year),
        header_desc: song.header_desc,
        play_count: parse_type(match song.play_count {
            Union::A(play_count) => play_count,
            Union::B(play_count) => play_count.to_string(),
        }),
        explicit: parse_bool(song.explicit_content),
        list_count: parse_type(song.list_count),
        list_type: song.list_type,
        list: song.list,
        music: more_info.music,
        album_id: more_info.album_id,
        album: more_info.album,
        label: more_info.label,
        label_url: more_info.label_url,
        song: more_info.song,
        origin: more_info.origin,
        is_dolby_content: more_info.is_dolby_content,
        _320kbps: parse_bool(more_info._320kbps),
        download_url: create_download_links(more_info.encrypted_media_url),
        album_url: more_info.album_url,
        duration: parse_type(more_info.duration),
        rights: more_info.rights,
        has_lyrics: parse_bool(more_info.has_lyrics),
        lyrics_id: more_info.lyrics_id,
        lyrics_snippet: more_info.lyrics_snippet,
        starred: parse_bool(more_info.starred),
        artist_map: artist_map_payload(more_info.artist_map),
        release_date: more_info.release_date,
        triller_available: more_info.triller_available,
        vcode: more_info.vcode,
        vlink: more_info.vlink,
        copyright_text: more_info.copyright_text,
    }
}

fn song_modules_payload(modules: SongModulesRequest) -> SongModulesResponse {
    SongModulesResponse {
        recommend: SongModulesRecommendResponse {
            title: modules.reco.title,
            subtitle: modules.reco.subtitle,
            source: "/song/recommend".to_string(),
            position: modules.reco.position,
            params: SongModulesRecommendParamsResponse {
                id: modules.reco.source_params.pid,
                lang: modules.reco.source_params.language,
            },
        },

        currently_trending: SongModulesCurrentlyTrendingResponse {
            title: modules.currently_trending.title,
            subtitle: modules.currently_trending.subtitle,
            source: "/get/trending".to_string(),
            position: modules.currently_trending.position,
            params: SongModulesTrendingParamsResponse {
                type_field: modules.currently_trending.source_params.entity_type,
                lang: modules.currently_trending.source_params.entity_language,
            },
        },

        songs_by_same_artists: SongModulesSongsBySameArtistsResponse {
            title: modules.songs_by_same_artists.title,
            subtitle: modules.songs_by_same_artists.subtitle,
            source: "/artist/top-songs".to_string(),
            position: modules.songs_by_same_artists.position,
            params: SongModulesArtistSongParamsResponse {
                artist_id: modules.songs_by_same_artists.source_params.artist_ids,
                song_id: modules.songs_by_same_artists.source_params.song_id,
                lang: modules.songs_by_same_artists.source_params.language,
            },
        },

        songs_by_same_actors: SongModulesSongsBySameActorsResponse {
            title: modules.songs_by_same_actors.title,
            subtitle: modules.songs_by_same_actors.subtitle,
            source: "/get/actor-top-songs".to_string(),
            position: modules.songs_by_same_actors.position,
            params: SongModulesActorSongParamsResponse {
                actor_id: modules.songs_by_same_actors.source_params.actor_ids,
                song_id: modules.songs_by_same_actors.source_params.song_id,
                lang: modules.songs_by_same_actors.source_params.language,
            },
        },

        artists: SongModulesArtistsResponse {
            title: modules.artists.title,
            subtitle: modules.artists.subtitle,
            source: modules.artists.source,
            position: modules.artists.position,
        },
    }
}
