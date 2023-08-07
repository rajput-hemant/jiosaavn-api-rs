use crate::{
    models::modules::{
        ChartResponse, DiscoverResponse, ModulePlaylistResponse, ModulesRequest, ModulesResponse,
        RadioResponse, ShowResponse, TopShowResponse, TrendingResponse,
    },
    utils::{create_image_links, parse_explicit_content},
};

use super::{album_paylaod::album_payload, artist_payload::artist_map_payload};

/// Create modules payload from modules request
///
/// ## Arguments
///
/// * `modules` - Modules request
///
/// ## Returns
///
/// * `ModulesResponse` - Modules payload
pub fn modules_paylod(modules: ModulesRequest) -> ModulesResponse {
    ModulesResponse {
        radio: modules
            .radio
            .featured_stations
            .into_iter()
            .map(|station| RadioResponse {
                id: station.id,
                name: station.title,
                subtitle: station.subtitle,
                type_field: station.type_field,
                image: create_image_links(&station.image),
                url: station.perma_url,
                explicit: parse_explicit_content(station.explicit_content),
                color: station.more_info.color,
                description: station.more_info.description,
                featured_station_type: station.more_info.featured_station_type,
                language: station.more_info.language,
                query: station.more_info.query,
                station_display_text: station.more_info.station_display_text,
            })
            .collect(),
        discover: modules
            .browse_discover
            .into_iter()
            .map(|discover| DiscoverResponse {
                id: discover.id,
                name: discover.title,
                subtitle: discover.subtitle,
                type_field: discover.type_field,
                image: create_image_links(&discover.image),
                url: discover.perma_url,
                available: discover.more_info.available,
                is_featured: discover.more_info.is_featured,
                genre: discover.more_info.sub_type,
                video_thumbnail: discover.more_info.video_thumbnail,
                video_url: discover.more_info.video_url,
            })
            .collect(),
        albums: modules
            .new_albums
            .into_iter()
            .map(|album| album_payload(album))
            .collect(),
        charts: modules
            .charts
            .into_iter()
            .map(|chart| ChartResponse {
                id: chart.id,
                name: chart.title,
                subtitle: chart.subtitle,
                type_field: chart.type_field,
                image: create_image_links(&chart.image),
                url: chart.perma_url,
                explicit: parse_explicit_content(chart.explicit_content),
                language: chart.language,
            })
            .collect(),
        shows: TopShowResponse {
            last_page: modules.top_shows.last_page,
            shows: modules
                .top_shows
                .shows
                .into_iter()
                .map(|show| ShowResponse {
                    id: show.id,
                    name: show.title,
                    subtitle: show.subtitle,
                    type_field: show.type_field,
                    image: create_image_links(&show.image),
                    url: show.perma_url,
                    explicit: parse_explicit_content(show.explicit_content),
                    release_date: show.more_info.release_date,
                    season_number: show.more_info.season_number.parse().unwrap_or_default(),
                    square_image: show.more_info.square_image,
                    year: show.more_info.year.parse().unwrap_or_default(),
                })
                .collect(),
        },
        trending: modules
            .new_trending
            .into_iter()
            .map(|trending| TrendingResponse {
                id: trending.id,
                name: trending.title,
                subtitle: trending.subtitle,
                type_field: trending.type_field,
                image: create_image_links(&trending.image),
                url: trending.perma_url,
                explicit: parse_explicit_content(trending.explicit_content),
                language: trending.language,
                artist_map: trending.more_info.artist_map.map(artist_map_payload),
                play_count: trending.play_count.parse().unwrap_or_default(),
                year: trending.year.parse().unwrap_or_default(),
                firstname: trending.more_info.firstname,
                release_date: trending.more_info.release_date,
                fan_count: trending
                    .more_info
                    .fan_count
                    .map(|f| f.replace(",", "").parse().unwrap_or_default()),
                follower_count: trending
                    .more_info
                    .follower_count
                    .map(|f| f.parse().unwrap_or_default()),
                song_count: trending
                    .more_info
                    .song_count
                    .map(|f| f.parse().unwrap_or_default()),
            })
            .collect(),
        playlists: modules
            .top_playlists
            .into_iter()
            .map(|playlist| ModulePlaylistResponse {
                id: playlist.id,
                name: playlist.title,
                subtitle: playlist.subtitle,
                type_field: playlist.type_field,
                image: create_image_links(&playlist.image),
                url: playlist.perma_url,
                explicit: parse_explicit_content(playlist.explicit_content),
                firstname: playlist.more_info.firstname,
                follower_count: playlist
                    .more_info
                    .follower_count
                    .parse()
                    .unwrap_or_default(),
                last_updated: playlist.more_info.last_updated.parse().unwrap_or_default(),
                song_count: playlist.more_info.song_count.parse().unwrap_or_default(),
                user_id: playlist.more_info.uid,
            })
            .collect(),
    }
}
