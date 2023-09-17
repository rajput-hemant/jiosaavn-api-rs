use serde_json::from_str;

use super::{albums_payload, songs_payload};
use crate::{
    models::{
        artist::{
            ArtistAlbumRequest, ArtistAlbumResponse, ArtistMapRequest, ArtistMapResponse,
            ArtistMiniRequest, ArtistMiniResponse, ArtistModule, ArtistModulesRequest,
            ArtistModulesResponse, ArtistPlaylistRequest, ArtistPlaylistResponse, ArtistRequest,
            ArtistResponse, ArtistSongsOrAlbumsRequest, ArtistSongsOrAlbumsResponse,
            ArtistTopSongsOrAlbums, SimilarArtistRequest, SimilarArtistResponse,
        },
        misc::Union,
    },
    utils::{create_image_links, parse_bool, parse_type},
};

/// Create artist payload from artist request
///
/// ## Arguments
///
/// * `artist` - Artist request
///
/// ## Returns
///
/// * `ArtistResponse` - Artist payload
pub fn artist_payload(artist: ArtistRequest) -> ArtistResponse {
    let a = artist;
    let (t_s, t_a, d, f, s, l, s_a) = (
        a.top_songs,
        a.top_albums,
        a.dedicated_artist_playlist,
        a.featured_artist_playlist,
        a.singles,
        a.latest_release,
        a.similar_artists,
    );

    ArtistResponse {
        id: a.artist_id,
        name: a.name,
        subtitle: a.subtitle,
        image: create_image_links(a.image),
        follower_count: parse_type(a.follower_count),
        type_field: a.type_field,
        is_verified: a.is_verified,
        dominant_language: a.dominant_language,
        dominant_type: a.dominant_type,
        top_songs: t_s.map_or(vec![], songs_payload),
        top_albums: t_a.map_or(vec![], albums_payload),
        dedicated_artist_playlist: d.map_or(vec![], artist_playlist_payload),
        featured_artist_playlist: f.map_or(vec![], artist_playlist_payload),
        singles: s.map_or(vec![], artist_songs_payload),
        latest_release: l.map_or(vec![], artist_songs_payload),
        similar_artists: similar_artists_payload(s_a),
        is_radio_present: a.is_radio_present,
        bio: from_str(&a.bio).unwrap(),
        dob: a.dob,
        fb: a.fb,
        twitter: a.twitter,
        wiki: a.wiki,
        urls: a.urls,
        available_languages: a.available_languages,
        fan_count: parse_type(a.fan_count),
        is_followed: a.is_followed,
        modules: artist_modules_payload(a.modules),
    }
}

/// Create payload artist songs
///
/// ## Arguments
///
/// * `songs` - Vector of artist songs
///
/// ## Returns
///
/// * `Vec<ArtistAlbumResponse>` - Vector of artist songs payload
pub fn artist_songs_payload(songs: Vec<ArtistAlbumRequest>) -> Vec<ArtistAlbumResponse> {
    songs
        .into_iter()
        .map(|s| ArtistAlbumResponse {
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
            music: s.more_info.music,
            query: s.more_info.query,
            text: s.more_info.text,
            song_count: parse_type(s.more_info.song_count),
            artist_map: artist_map_payload(s.more_info.artist_map),
        })
        .collect()
}

/// Create payload for similar artists
///
/// ## Arguments
///
/// * `similar_artist` - Vector of similar artists
///
/// ## Returns
///
/// * `Vec<SimilarArtistResponse>` - Vector of similar artists payload
pub fn similar_artists_payload(
    similar_artist: Vec<SimilarArtistRequest>,
) -> Vec<SimilarArtistResponse> {
    similar_artist
        .into_iter()
        .map(|s| SimilarArtistResponse {
            id: s.id,
            name: s.name,
            roles: from_str(&s.roles).unwrap(),
            aka: s.aka,
            fb: s.fb,
            twitter: s.twitter,
            wiki: s.wiki,
            similar: from_str(&s.similar).unwrap(),
            dob: s.dob,
            image_url: create_image_links(s.image_url),
            search_keywords: s.search_keywords,
            primary_artist_id: s.primary_artist_id,
            languages: from_str(&s.languages).unwrap(),
            url: s.perma_url,
            type_field: s.type_field,
            is_radio_present: s.is_radio_present,
            dominant_type: s.dominant_type,
        })
        .collect()
}

/// Create payload for artist map
///
/// ## Arguments
///
/// * `artist_map` - Artist map request
///
/// ## Returns
///
/// * `ArtistMapResponse` - Artist map payload
pub fn artist_map_payload(artist_map: ArtistMapRequest) -> ArtistMapResponse {
    let a = artist_map;
    let (a, f, p) = (a.artists, a.featured_artists, a.primary_artists);

    ArtistMapResponse {
        artists: artist_mini_payload(a),
        featured_artists: artist_mini_payload(f),
        primary_artists: artist_mini_payload(p),
    }
}

/// Create payload for artist mini object
///
/// ## Arguments
///
/// * `artist_mini` - Vector of artist mini
///
/// ## Returns
///
/// * `Vec<ArtistMiniResponse>` - Vector of artist mini object payload
pub fn artist_mini_payload(artist_mini: Vec<ArtistMiniRequest>) -> Vec<ArtistMiniResponse> {
    artist_mini
        .into_iter()
        .map(|a| ArtistMiniResponse {
            id: a.id,
            name: a.name,
            role: a.role,
            image: create_image_links(a.image),
            type_field: a.type_field,
            url: a.perma_url,
        })
        .collect()
}

/// Create payload for artist modules
///
/// ## Arguments
///
/// * `modules` - Artist modules request
///
/// ## Returns
///
/// * `ArtistModulesResponse` - Artist modules payload
pub fn artist_modules_payload(modules: ArtistModulesRequest) -> ArtistModulesResponse {
    let m = modules;
    let t_s = m.top_songs.unwrap_or_default();
    let t_a = m.top_albums.unwrap_or_default();
    let l = m.latest_release.unwrap_or_default();
    let f = m.featured_artist_playlist.unwrap_or_default();
    let d = m.dedicated_artist_playlist.unwrap_or_default();
    let s_a = m.similar_artists.unwrap_or_default();
    let s = m.singles.unwrap_or_default();

    ArtistModulesResponse {
        top_songs: ArtistModule {
            title: t_s.title,
            subtitle: t_s.subtitle,
            position: t_s.position,
            source: "/artist/top-songs".to_string(),
        },

        top_albums: ArtistModule {
            title: t_a.title,
            subtitle: t_a.subtitle,
            source: "top_albums|topAlbums".to_string(),
            position: t_a.position,
        },

        dedicated_artist_playlist: ArtistModule {
            title: d.title,
            subtitle: d.subtitle,
            position: d.position,
            source: "dedicated_artist_playlist|dedicatedArtistPlaylist".to_string(),
        },

        featured_artist_playlist: ArtistModule {
            title: f.title,
            subtitle: f.subtitle,
            position: f.position,
            source: "featured_artist_playlist|featuredArtistPlaylist".to_string(),
        },

        latest_release: ArtistModule {
            title: l.title,
            subtitle: l.subtitle,
            position: l.position,
            source: "latest_release|latestRelease".to_string(),
        },

        similar_artists: ArtistModule {
            title: s_a.title,
            subtitle: s_a.subtitle,
            position: s_a.position,
            source: "similar_artists|similarArtists".to_string(),
        },

        singles: ArtistModule {
            title: s.title,
            subtitle: s.subtitle,
            position: s.position,
            source: "singles".to_string(),
        },
    }
}

/// Create payload for artist playlist
///
/// ## Arguments
///
/// * `playlist` - Vector of artist playlist
///
/// ## Returns
///
/// * `Vec<ArtistPlaylistResponse>` - Vector of artist playlist payload
pub fn artist_playlist_payload(
    playlist: Vec<ArtistPlaylistRequest>,
) -> Vec<ArtistPlaylistResponse> {
    playlist
        .into_iter()
        .map(|p| {
            let m = p.more_info;
            ArtistPlaylistResponse {
                id: p.id,
                name: p.title,
                subtitle: p.subtitle,
                type_field: p.type_field,
                image: create_image_links(p.image),
                url: p.perma_url,
                explicit: parse_bool(p.explicit_content),
                user_id: m.uid,
                firstname: m.firstname,
                lastname: m.lastname,
                language: m.language,
                artist_name: m.artist_name.map(|a| a[0].to_owned()),
                entity_type: m.entity_type,
                entity_sub_type: m.entity_sub_type,
                video_available: m.video_available,
                is_dolby_content: m.is_dolby_content,
                sub_types: m.sub_types,
                images: m.images,
                song_count: parse_type(m.song_count),
            }
        })
        .collect()
}

/// Create payload for artist songs or albums
///
/// ## Arguments
///
/// * `artist_songs_albums` - Artist songs or albums request
///
/// ## Returns
///
/// * `ArtistSongsOrAlbumsResponse` - Artist songs or albums payload
pub fn artist_songs_or_albums_payload(
    artist_songs_albums: ArtistSongsOrAlbumsRequest,
) -> ArtistSongsOrAlbumsResponse {
    let i = artist_songs_albums;
    ArtistSongsOrAlbumsResponse {
        id: i.artist_id,
        name: i.name,
        image: create_image_links(i.image),
        follower_count: parse_type(i.follower_count),
        type_field: i.type_field,
        is_verified: i.is_verified,
        dominant_language: i.dominant_language,
        dominant_type: i.dominant_type,
        top_songs: i.top_songs.map(|top| ArtistTopSongsOrAlbums {
            total: top.total,
            last_page: top.last_page,
            songs: top.songs.map(songs_payload),
            albums: None,
        }),
        top_albums: i.top_albums.map(|top| ArtistTopSongsOrAlbums {
            total: top.total,
            last_page: top.last_page,
            songs: None,
            albums: top.albums.map(albums_payload),
        }),
    }
}
