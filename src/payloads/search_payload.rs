use crate::{
    models::{
        album::{AlbumRequest, AlbumResponse},
        search::{
            AllSearchRequest, AllSearchResponse, SearchAlbumRequest, SearchAlbumResponse,
            SearchArtistRequest, SearchArtistResponse, SearchPlaylistRequest,
            SearchPlaylistResponse, SearchResponse, SearchShowRequest, SearchShowResponse,
            SearchSongRequest, SearchSongResponse, TSearchResponse, TopQueryRequest,
            TopQueryRespone, TopSearchesRequest, TopSearchesResponse,
        },
        song::{SongRequest, SongResponse},
    },
    utils::{create_image_links, parse_explicit_content, parse_type},
};

use super::{album_payload, song_payload};

/// Create all search payload from all search request
///
/// ## Arguments
///
/// * `results` - All search request
///
/// ## Returns
///
/// * `AllSearchResponse` - All search payload
pub fn all_search_payload(results: AllSearchRequest) -> AllSearchResponse {
    AllSearchResponse {
        topquery: SearchResponse {
            position: results.topquery.position,
            results: results
                .topquery
                .data
                .into_iter()
                .map(top_query_payload)
                .collect(),
        },
        albums: SearchResponse {
            position: results.albums.position,
            results: results
                .albums
                .data
                .into_iter()
                .map(search_album_payload)
                .collect(),
        },
        songs: SearchResponse {
            position: results.songs.position,
            results: results
                .songs
                .data
                .into_iter()
                .map(search_song_payload)
                .collect(),
        },
        artists: SearchResponse {
            position: results.artists.position,
            results: results
                .artists
                .data
                .into_iter()
                .map(search_artist_payload)
                .collect(),
        },
        playlists: SearchResponse {
            position: results.playlists.position,
            results: results
                .playlists
                .data
                .into_iter()
                .map(search_playlist_payload)
                .collect(),
        },
        shows: SearchResponse {
            position: results.shows.position,
            results: results
                .shows
                .data
                .into_iter()
                .map(search_show_payload)
                .collect(),
        },
    }
}

/// Create song search payload from song search request
///
/// ## Arguments
///
/// * `results` - Song search request
///
/// ## Returns
///
/// * `TSearchResponse<SongResponse>` - Song search payload
pub fn song_search_payload(results: TSearchResponse<SongRequest>) -> TSearchResponse<SongResponse> {
    TSearchResponse {
        total: results.total,
        start: results.start,
        results: results.results.into_iter().map(song_payload).collect(),
    }
}

/// Create album search payload from album search request
///
/// ## Arguments
///
/// * `results` - Album search request
///
/// ## Returns
///
/// * `TSearchResponse<AlbumResponse>` - Album search payload
pub fn album_search_payload(
    results: TSearchResponse<AlbumRequest>,
) -> TSearchResponse<AlbumResponse> {
    TSearchResponse {
        total: results.total,
        start: results.start,
        results: results.results.into_iter().map(album_payload).collect(),
    }
}

/// Create playlist search payload from playlist search request
///
/// ## Arguments
///
/// * `results` - Playlist search request
///
/// ## Returns
///
/// * `TSearchResponse<SearchPlaylistResponse>` - Playlist search payload
pub fn playlist_search_payload(
    results: TSearchResponse<SearchPlaylistRequest>,
) -> TSearchResponse<SearchPlaylistResponse> {
    TSearchResponse {
        total: results.total,
        start: results.start,
        results: results
            .results
            .into_iter()
            .map(search_playlist_payload)
            .collect(),
    }
}

/// Create artist search payload from artist search request
///
/// ## Arguments
///
/// * `results` - Artist search request
///
/// ## Returns
///
/// * `TSearchResponse<SearchArtistResponse>` - Artist search payload
pub fn artist_search_payload(
    results: TSearchResponse<SearchArtistRequest>,
) -> TSearchResponse<SearchArtistResponse> {
    TSearchResponse {
        total: results.total,
        start: results.start,
        results: results
            .results
            .into_iter()
            .map(search_artist_payload)
            .collect(),
    }
}

/// Create top query payload from top query request
///
/// ## Arguments
///
/// * `top_query` - Top query request
///
/// ## Returns
///
/// * `TopQueryRespone` - Top query payload
pub fn top_query_payload(top_query: TopQueryRequest) -> TopQueryRespone {
    TopQueryRespone {
        id: top_query.id,
        name: top_query.title,
        subtitle: top_query.subtitle,
        type_field: top_query.type_field,
        image: create_image_links(top_query.image),
        url: top_query.perma_url,
        explicit: parse_explicit_content(top_query.explicit_content),
        description: top_query.description,
        music: top_query.more_info.music,
        year: top_query.more_info.year.map(parse_type),
        is_movie: top_query.more_info.is_movie.map(parse_type),
        language: top_query.more_info.language,
        song_ids: top_query.more_info.song_pids,
        primary_artists: top_query.more_info.primary_artists,
        score: top_query.more_info.score.map(parse_type),
        singers: top_query.more_info.singers,
    }
}

/// Create top searches payload from top searches request
///
/// ## Arguments
///
/// * `searches` - Top searches request
///
/// ## Returns
///
/// * `Vec<TopSearchesResponse>` - Top searches payload
pub fn top_searches_payload(searches: Vec<TopSearchesRequest>) -> Vec<TopSearchesResponse> {
    searches
        .into_iter()
        .map(|i| TopSearchesResponse {
            id: i.id,
            name: i.title,
            subtitle: i.subtitle,
            image: create_image_links(i.image),
            explicit: parse_explicit_content(i.explicit_content),
            type_field: i.type_field,
            url: i.perma_url,
        })
        .collect()
}

fn search_album_payload(album: SearchAlbumRequest) -> SearchAlbumResponse {
    SearchAlbumResponse {
        id: album.id,
        name: album.title,
        subtitle: album.subtitle,
        type_field: album.type_field,
        image: create_image_links(album.image),
        url: album.perma_url,
        explicit: parse_explicit_content(album.explicit_content),
        description: album.description,
        music: album.more_info.music,
        year: parse_type(album.more_info.year),
        is_movie: parse_type(album.more_info.is_movie),
        language: album.more_info.language,
        song_ids: album.more_info.song_pids,
    }
}

fn search_artist_payload(artist: SearchArtistRequest) -> SearchArtistResponse {
    SearchArtistResponse {
        id: artist.id,
        name: if artist.title.is_none() {
            artist.name
        } else {
            artist.title
        },
        image: artist.image,
        extra: artist.extra,
        role: artist.role,
        type_field: artist.type_field,
        is_radio_present: artist.is_radio_present,
        entity: artist.entity,
        description: artist.description,
        position: artist.position,
    }
}

fn search_song_payload(song: SearchSongRequest) -> SearchSongResponse {
    SearchSongResponse {
        id: song.id,
        name: song.title,
        subtitle: song.subtitle,
        type_field: song.type_field,
        image: create_image_links(song.image),
        url: song.perma_url,
        explicit: parse_explicit_content(song.explicit_content),
        description: song.description,
        score: parse_type(song.more_info.score),
        primary_artists: song.more_info.primary_artists,
        singers: song.more_info.singers,
        language: song.more_info.language,
    }
}

fn search_show_payload(show: SearchShowRequest) -> SearchShowResponse {
    SearchShowResponse {
        id: show.id,
        name: show.title,
        subtitle: show.subtitle,
        type_field: show.type_field,
        image: create_image_links(show.image),
        url: show.perma_url,
        explicit: parse_explicit_content(show.explicit_content),
        description: show.description,
        season_number: show.more_info.season_number,
    }
}

/// Create playlist search payload from playlist search request
///
/// ## Arguments
///
/// * `playlist` - Playlist search request
///
/// ## Returns
///
/// * `SearchPlaylistResponse` - Playlist search payload
pub fn search_playlist_payload(playlist: SearchPlaylistRequest) -> SearchPlaylistResponse {
    SearchPlaylistResponse {
        id: playlist.id,
        name: playlist.title,
        subtitle: playlist.subtitle,
        type_field: playlist.type_field,
        image: create_image_links(playlist.image),
        url: playlist.perma_url,
        explicit: parse_explicit_content(playlist.explicit_content),
        description: playlist.description,
        firstname: playlist.more_info.firstname,
        lastname: playlist.more_info.lastname,
        language: playlist.more_info.language,
        entity_type: playlist.more_info.entity_type,
        last_updated: playlist.more_info.last_updated,
        song_count: playlist.more_info.song_count,
        user_id: playlist.more_info.uid,
    }
}
