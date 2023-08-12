use crate::{
    models::artist::{
        ArtistAlbumsRequest, ArtistAlbumsResponse, ArtistMapResponse, ArtistMiniRequest,
        ArtistMiniResponse, ArtistRequest, ArtistResponse, ArtistSongsRequest, ArtistSongsResponse,
        ArtistTopAlbums, ArtistTopSongs, SimilarArtistRequest, SimilarArtistResponse, ArtistMapRequest,
    },
    utils::{create_image_links, parse_type},
};

use super::{album_payload, song_payload};

pub fn artist_payload(artist: ArtistRequest) -> ArtistResponse {
    ArtistResponse {
        id: artist.artist_id,
        name: artist.name,
        subtitle: artist.subtitle,
        image: create_image_links(artist.image),
        follower_count: parse_type(artist.follower_count),
        type_field: artist.type_field,
        is_verified: artist.is_verified,
        dominant_language: artist.dominant_language,
        dominant_type: artist.dominant_type,
        top_songs: artist.top_songs.into_iter().map(song_payload).collect(),
        top_albums: artist.top_albums.into_iter().map(album_payload).collect(),
        similar_artists: artist
            .similar_artists
            .into_iter()
            .map(similar_artist_payload)
            .collect(),
        is_radio_present: artist.is_radio_present,
        bio: artist.bio,
        dob: artist.dob,
        fb: artist.fb,
        twitter: artist.twitter,
        wiki: artist.wiki,
        urls: artist.urls,
        available_languages: artist.available_languages,
        fan_count: parse_type(artist.fan_count),
        is_followed: artist.is_followed,
    }
}

pub fn similar_artist_payload(similar_artist: SimilarArtistRequest) -> SimilarArtistResponse {
    SimilarArtistResponse {
        id: similar_artist.id,
        name: similar_artist.name,
        roles: similar_artist.roles,
        aka: similar_artist.aka,
        fb: similar_artist.fb,
        twitter: similar_artist.twitter,
        wiki: similar_artist.wiki,
        similar: similar_artist.similar,
        dob: similar_artist.dob,
        image_url: create_image_links(similar_artist.image_url),
        search_keywords: similar_artist.search_keywords,
        primary_artist_id: similar_artist.primary_artist_id,
        combine_artist_pages: similar_artist.combine_artist_pages,
        replace_with_primary_artists: similar_artist.replace_with_primary_artists,
        languages: similar_artist.languages,
        url: similar_artist.perma_url,
        type_field: similar_artist.type_field,
        is_radio_present: similar_artist.is_radio_present,
        dominant_type: similar_artist.dominant_type,
    }
}

pub fn artist_map_payload(artist_map: ArtistMapRequest) -> ArtistMapResponse {
    ArtistMapResponse {
        artists: artist_map
            .artists
            .into_iter()
            .map(artist_mini_payload)
            .collect(),
        featured_artists: artist_map
            .featured_artists
            .into_iter()
            .map(artist_mini_payload)
            .collect(),
        primary_artists: artist_map
            .primary_artists
            .into_iter()
            .map(artist_mini_payload)
            .collect(),
    }
}

pub fn artist_mini_payload(artist_mini: ArtistMiniRequest) -> ArtistMiniResponse {
    ArtistMiniResponse {
        id: artist_mini.id,
        name: artist_mini.name,
        role: artist_mini.role,
        image: artist_mini.image,
        type_field: artist_mini.type_field,
        url: artist_mini.perma_url,
    }
}

pub fn artist_songs_payload(artist_songs: ArtistSongsRequest) -> ArtistSongsResponse {
    ArtistSongsResponse {
        id: artist_songs.artist_id,
        name: artist_songs.name,
        image: create_image_links(artist_songs.image),
        follower_count: parse_type(artist_songs.follower_count),
        type_field: artist_songs.type_field,
        is_verified: artist_songs.is_verified,
        dominant_language: artist_songs.dominant_language,
        dominant_type: artist_songs.dominant_type,
        top_songs: ArtistTopSongs {
            total: artist_songs.top_songs.total,
            last_page: artist_songs.top_songs.last_page,
            songs: artist_songs
                .top_songs
                .songs
                .into_iter()
                .map(song_payload)
                .collect(),
        },
    }
}

pub fn artist_albums_payload(artist_albums: ArtistAlbumsRequest) -> ArtistAlbumsResponse {
    ArtistAlbumsResponse {
        id: artist_albums.artist_id,
        name: artist_albums.name,
        image: create_image_links(artist_albums.image),
        follower_count: parse_type(artist_albums.follower_count),
        type_field: artist_albums.type_field,
        is_verified: artist_albums.is_verified,
        dominant_language: artist_albums.dominant_language,
        dominant_type: artist_albums.dominant_type,
        top_albums: ArtistTopAlbums {
            total: artist_albums.top_albums.total,
            last_page: artist_albums.top_albums.last_page,
            albums: artist_albums
                .top_albums
                .albums
                .into_iter()
                .map(album_payload)
                .collect(),
        },
    }
}
