use crate::{
    models::modules::{
        ArtistRecoRequest, ArtistRecoResponse, ChartRequest, ChartResponse, CityModRequest,
        CityModRequestMultipleTune, CityModResponse, CityModResponseMultipleTune, DiscoverRequest,
        DiscoverResonse, Module, ModulePlaylistRequest, ModulePlaylistResponse, ModulesRequest,
        ModulesResponse, PromoRequest, PromoResponse, RadioRequest, RadioResponse, TagMixRequest,
        TagMixResponse, TrendingRequest, TrendingResponse,
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
pub fn modules_payload(modules: ModulesRequest) -> ModulesResponse {
    ModulesResponse {
        artist_recos: Module {
            title: modules.modules.artist_recos.title,
            subtitle: modules.modules.artist_recos.subtitle,
            featured_text: modules.modules.artist_recos.featured_text,
            data: modules
                .artist_recos
                .into_iter()
                .map(artist_reco_payload)
                .collect(),
        },
        discover: modules
            .browse_discover
            .into_iter()
            .map(discover_payload)
            .collect(),

        charts: Module {
            title: modules.modules.charts.title,
            subtitle: modules.modules.charts.subtitle,
            featured_text: modules.modules.charts.featured_text,
            data: modules.charts.into_iter().map(chart_payload).collect(),
        },
        city_mod: Module {
            title: modules.modules.city_mod.title,
            subtitle: modules.modules.city_mod.subtitle,
            featured_text: modules.modules.city_mod.featured_text,
            data: modules.city_mod.into_iter().map(city_mod_payload).collect(),
        },
        global_config: modules.global_config,
        albums: Module {
            title: modules.modules.new_albums.title,
            subtitle: modules.modules.new_albums.subtitle,
            featured_text: modules.modules.new_albums.featured_text,
            data: modules.new_albums.into_iter().map(album_payload).collect(),
        },
        trending: Module {
            title: modules.modules.new_trending.title,
            subtitle: modules.modules.new_trending.subtitle,
            featured_text: modules.modules.new_trending.featured_text,
            data: modules
                .new_trending
                .into_iter()
                .map(trending_payload)
                .collect(),
        },
        playlists: Module {
            title: modules.modules.top_playlists.title,
            subtitle: modules.modules.top_playlists.subtitle,
            featured_text: modules.modules.top_playlists.featured_text,
            data: modules
                .top_playlists
                .into_iter()
                .map(module_playlist_payload)
                .collect(),
        },
        mixes: Module {
            title: modules.modules.tag_mixes.title,
            subtitle: modules.modules.tag_mixes.subtitle,
            featured_text: modules.modules.tag_mixes.featured_text,
            data: modules
                .tag_mixes
                .into_iter()
                .map(tag_mixes_payload)
                .collect(),
        },
        radio: Module {
            title: modules.modules.radio.title,
            subtitle: modules.modules.radio.subtitle,
            featured_text: modules.modules.radio.featured_text,
            data: modules.radio.into_iter().map(radio_payload).collect(),
        },
        promo_107: Module {
            title: modules.modules.promo_107.title,
            subtitle: modules.modules.promo_107.subtitle,
            featured_text: modules.modules.promo_107.featured_text,
            data: modules.promo_107.into_iter().map(promo_payload).collect(),
        },
        promo_112: Module {
            title: modules.modules.promo_112.title,
            subtitle: modules.modules.promo_112.subtitle,
            featured_text: modules.modules.promo_112.featured_text,
            data: modules.promo_112.into_iter().map(promo_payload).collect(),
        },
        promo_113: Module {
            title: modules.modules.promo_113.title,
            subtitle: modules.modules.promo_113.subtitle,
            featured_text: modules.modules.promo_113.featured_text,
            data: modules.promo_113.into_iter().map(promo_payload).collect(),
        },
        promo_114: Module {
            title: modules.modules.promo_114.title,
            subtitle: modules.modules.promo_114.subtitle,
            featured_text: modules.modules.promo_114.featured_text,
            data: modules.promo_114.into_iter().map(promo_payload).collect(),
        },
        promo_116: Module {
            title: modules.modules.promo_116.title,
            subtitle: modules.modules.promo_116.subtitle,
            featured_text: modules.modules.promo_116.featured_text,
            data: modules.promo_116.into_iter().map(promo_payload).collect(),
        },
        promo_118: Module {
            title: modules.modules.promo_118.title,
            subtitle: modules.modules.promo_118.subtitle,
            featured_text: modules.modules.promo_118.featured_text,
            data: modules.promo_118.into_iter().map(promo_payload).collect(),
        },
        promo_176: Module {
            title: modules.modules.promo_176.title,
            subtitle: modules.modules.promo_176.subtitle,
            featured_text: modules.modules.promo_176.featured_text,
            data: modules.promo_176.into_iter().map(promo_payload).collect(),
        },
        promo_185: Module {
            title: modules.modules.promo_185.title,
            subtitle: modules.modules.promo_185.subtitle,
            featured_text: modules.modules.promo_185.featured_text,
            data: modules.promo_185.into_iter().map(promo_payload).collect(),
        },
        promo_49: Module {
            title: modules.modules.promo_49.title,
            subtitle: modules.modules.promo_49.subtitle,
            featured_text: modules.modules.promo_49.featured_text,
            data: modules.promo_49.into_iter().map(promo_payload).collect(),
        },
        promo_68: Module {
            title: modules.modules.promo_68.title,
            subtitle: modules.modules.promo_68.subtitle,
            featured_text: modules.modules.promo_68.featured_text,
            data: modules.promo_68.into_iter().map(promo_payload).collect(),
        },
        promo_76: Module {
            title: modules.modules.promo_76.title,
            subtitle: modules.modules.promo_76.subtitle,
            featured_text: modules.modules.promo_76.featured_text,
            data: modules.promo_76.into_iter().map(promo_payload).collect(),
        },
        promo_90: Module {
            title: modules.modules.promo_90.title,
            subtitle: modules.modules.promo_90.subtitle,
            featured_text: modules.modules.promo_90.featured_text,
            data: modules.promo_90.into_iter().map(promo_payload).collect(),
        },
    }
}

fn artist_reco_payload(artist: ArtistRecoRequest) -> ArtistRecoResponse {
    ArtistRecoResponse {
        id: artist.id,
        image: create_image_links(&artist.image),
        name: artist.title,
        type_field: artist.type_field,
        url: artist.perma_url,
        subtitle: artist.subtitle,
        explicit: parse_explicit_content(artist.explicit_content),
        featured_station_type: artist.more_info.featured_station_type,
        query: artist.more_info.query,
        station_display_text: artist.more_info.station_display_text,
    }
}

fn discover_payload(discover: DiscoverRequest) -> DiscoverResonse {
    DiscoverResonse {
        id: discover.id,
        image: create_image_links(&discover.image),
        name: discover.title,
        type_field: discover.type_field,
        url: discover.perma_url,
        subtitle: discover.subtitle,
        explicit: parse_explicit_content(discover.explicit_content),
        badge: discover.more_info.badge,
        is_featured: discover.more_info.is_featured.parse().unwrap_or_default(),
        sub_type: discover.more_info.sub_type,
        video_thumbnail: discover.more_info.video_thumbnail,
        video_url: discover.more_info.video_url,
    }
}

fn chart_payload(chart: ChartRequest) -> ChartResponse {
    let more_info = chart.more_info.unwrap_or_default();
    ChartResponse {
        id: chart.id,
        image: create_image_links(&chart.image),
        name: chart.title,
        type_field: chart.type_field,
        url: chart.perma_url,
        subtitle: chart.subtitle,
        explicit: chart.explicit_content.map(parse_explicit_content),
        count: chart.count,
        listname: chart.listname,
        language: chart.language,
        firstname: more_info.firstname,
        song_count: more_info.song_count,
    }
}

fn city_mod_payload(city_mod: CityModRequest) -> CityModResponse {
    CityModResponse {
        id: city_mod.id,
        image: create_image_links(&city_mod.image),
        url: city_mod.perma_url,
        subtitle: city_mod.subtitle,
        name: city_mod.title,
        type_field: city_mod.type_field,
        explicit: city_mod.explicit_content.parse().unwrap_or_default(),
        query: city_mod.more_info.query,
        album_id: city_mod.more_info.album_id,
        featured_station_type: city_mod.more_info.featured_station_type,
        multiple_tunes: city_mod.more_info.multiple_tunes.map(|tunes| {
            tunes
                .into_iter()
                .map(city_mod_multiple_tunes_payload)
                .collect()
        }),
    }
}

fn city_mod_multiple_tunes_payload(
    tune: CityModRequestMultipleTune,
) -> CityModResponseMultipleTune {
    CityModResponseMultipleTune {
        id: tune.id,
        subtype: tune.subtype,
        name: tune.title,
        type_field: tune.type_field,
    }
}

fn trending_payload(trending: TrendingRequest) -> TrendingResponse {
    TrendingResponse {
        id: trending.id,
        name: trending.title,
        subtitle: trending.subtitle,
        field: trending.type_field,
        url: trending.perma_url,
        image: create_image_links(&trending.image),
        language: trending.language,
        year: trending.year.parse().unwrap_or_default(),
        play_count: trending.play_count.parse().unwrap_or_default(),
        explicit: trending.explicit_content.parse().unwrap_or_default(),
        list_count: trending.list_count.parse().unwrap_or_default(),
        list_type: trending.list_type,
        list: trending.list,
        release_date: trending.more_info.release_date,
        song_count: trending
            .more_info
            .song_count
            .map(|s| s.parse().unwrap_or_default()),
        artist_map: trending.more_info.artist_map.map(artist_map_payload),
        music: trending.more_info.music,
        album_id: trending.more_info.album_id,
        album: trending.more_info.album,
        label: trending.more_info.label,
        origin: trending.more_info.origin,
        is_dolby_content: trending.more_info.is_dolby_content,
        _320kbps: trending
            .more_info
            ._320kbps
            .map(|s| s.parse().unwrap_or_default()),
        encrypted_media_url: trending.more_info.encrypted_media_url,
        encrypted_cache_url: trending.more_info.encrypted_cache_url,
        album_url: trending.more_info.album_url,
        duration: trending
            .more_info
            .duration
            .map(|s| s.parse().unwrap_or_default()),
        rights: trending.more_info.rights,
        cache_state: trending.more_info.cache_state,
        has_lyrics: trending.more_info.has_lyrics,
        lyrics_snippet: trending.more_info.lyrics_snippet,
        copyright_text: trending.more_info.copyright_text,
        label_url: trending.more_info.label_url,
        lyrics_id: trending.more_info.lyrics_id,
        firstname: trending.more_info.firstname,
        follower_count: trending
            .more_info
            .follower_count
            .map(|s| s.parse().unwrap_or_default()),
        fan_count: trending
            .more_info
            .fan_count
            .map(|s| s.parse().unwrap_or_default()),
    }
}

fn module_playlist_payload(playlist: ModulePlaylistRequest) -> ModulePlaylistResponse {
    ModulePlaylistResponse {
        id: playlist.id,
        name: playlist.title,
        subtitle: playlist.subtitle,
        type_field: playlist.type_field,
        image: create_image_links(&playlist.image),
        url: playlist.perma_url,
        explicit: playlist.explicit_content.parse().unwrap_or_default(),
        song_count: playlist.more_info.song_count.parse().unwrap_or_default(),
        follower_count: playlist
            .more_info
            .follower_count
            .parse()
            .unwrap_or_default(),
        last_updated: playlist.more_info.last_updated.parse().unwrap_or_default(),
        firstname: playlist.more_info.firstname,
        user_id: playlist.more_info.uid,
    }
}

fn tag_mixes_payload(mixes: TagMixRequest) -> TagMixResponse {
    TagMixResponse {
        id: mixes.id,
        name: mixes.title,
        subtitle: mixes.subtitle,
        type_field: mixes.type_field,
        image: create_image_links(&mixes.image),
        url: mixes.perma_url,
        explicit: mixes.explicit_content.parse().unwrap_or_default(),
        language: mixes.language,
        list_count: mixes.list_count.parse().unwrap_or_default(),
        list_type: mixes.list_type,
        list: mixes.list,
        play_count: mixes.play_count.parse().unwrap_or_default(),
        year: mixes.year.parse().unwrap_or_default(),
        firstname: mixes.more_info.firstname,
        lastname: mixes.more_info.lastname,
    }
}

fn radio_payload(radio: RadioRequest) -> RadioResponse {
    RadioResponse {
        id: radio.id,
        name: radio.title,
        subtitle: radio.subtitle,
        type_field: radio.type_field,
        image: create_image_links(&radio.image),
        url: radio.perma_url,
        explicit: radio.explicit_content.parse().unwrap_or_default(),
        description: radio.more_info.description,
        featured_station_type: radio.more_info.featured_station_type,
        query: radio.more_info.query,
        color: radio.more_info.color,
        language: radio.more_info.language,
        station_display_text: radio.more_info.station_display_text,
    }
}

fn promo_payload(promo: PromoRequest) -> PromoResponse {
    PromoResponse {
        id: promo.id,
        image: create_image_links(&promo.image),
        url: promo.perma_url,
        subtitle: promo.subtitle,
        name: promo.title,
        type_field: promo.type_field,
        language: promo.language,
        list: promo.list,
        list_count: promo.list_count.map(|s| s.parse().unwrap_or_default()),
        list_type: promo.list_type,
        play_count: promo.play_count.map(|s| s.parse().unwrap_or_default()),
        year: promo.year.map(|s| s.parse().unwrap_or_default()),
        explicit: promo.explicit_content.parse().unwrap_or_default(),
        square_image: promo.more_info.square_image,
        editorial_language: promo.more_info.editorial_language,
        position: promo
            .more_info
            .position
            .map(|s| s.parse().unwrap_or_default()),
        release_year: promo.more_info.release_year,
    }
}
