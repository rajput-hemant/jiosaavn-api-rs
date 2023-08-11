use crate::{
    models::modules::{
        ArtistRecoRequest, ArtistRecoResponse, ChartRequest, ChartResponse, CityModRequest,
        CityModRequestMultipleTune, CityModResponse, CityModResponseMultipleTune, DiscoverRequest,
        DiscoverResonse, Module, ModulePlaylistRequest, ModulePlaylistResponse, ModulesRequest,
        ModulesResponse, PromoRequest, PromoResponse, RadioRequest, RadioResponse, TagMixRequest,
        TagMixResponse, TrendingRequest, TrendingResponse,
    },
    utils::{create_image_links, parse_explicit_content, parse_type},
};

use super::album_paylaod::album_payload;

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
        city_mod: modules.city_mod.map(|city_mod| {
            let more_info = modules.modules.city_mod.unwrap_or_default();
            Module {
                title: more_info.title,
                subtitle: more_info.subtitle,
                featured_text: more_info.featured_text,
                data: city_mod.into_iter().map(city_mod_payload).collect(),
            }
        }),
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
        image: create_image_links(artist.image),
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
        image: create_image_links(discover.image),
        name: discover.title,
        type_field: discover.type_field,
        url: discover.perma_url,
        subtitle: discover.subtitle,
        explicit: parse_explicit_content(discover.explicit_content),
        badge: discover.more_info.badge,
        is_featured: parse_type(discover.more_info.is_featured),
        sub_type: discover.more_info.sub_type,
        video_thumbnail: discover.more_info.video_thumbnail,
        video_url: discover.more_info.video_url,
    }
}

fn chart_payload(chart: ChartRequest) -> ChartResponse {
    let more_info = chart.more_info.unwrap_or_default();
    ChartResponse {
        id: chart.id,
        image: create_image_links(chart.image),
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
        image: create_image_links(city_mod.image),
        url: city_mod.perma_url,
        subtitle: city_mod.subtitle,
        name: city_mod.title,
        type_field: city_mod.type_field,
        explicit: parse_type(city_mod.explicit_content),
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
        image: create_image_links(trending.image),
        language: trending.language,
        year: parse_type(trending.year),
        play_count: parse_type(trending.play_count),
        explicit: parse_type(trending.explicit_content),
        list_count: parse_type(trending.list_count),
        list_type: trending.list_type,
        list: trending.list,
    }
}

fn module_playlist_payload(playlist: ModulePlaylistRequest) -> ModulePlaylistResponse {
    ModulePlaylistResponse {
        id: playlist.id,
        name: playlist.title,
        subtitle: playlist.subtitle,
        type_field: playlist.type_field,
        image: create_image_links(playlist.image),
        url: playlist.perma_url,
        explicit: parse_type(playlist.explicit_content),
        song_count: parse_type(playlist.more_info.song_count),
        follower_count: playlist
            .more_info
            .follower_count
            .parse()
            .unwrap_or_default(),
        last_updated: parse_type(playlist.more_info.last_updated),
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
        image: create_image_links(mixes.image),
        url: mixes.perma_url,
        explicit: parse_type(mixes.explicit_content),
        language: mixes.language,
        list_count: parse_type(mixes.list_count),
        list_type: mixes.list_type,
        list: mixes.list,
        play_count: parse_type(mixes.play_count),
        year: parse_type(mixes.year),
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
        image: create_image_links(radio.image),
        url: radio.perma_url,
        explicit: parse_type(radio.explicit_content),
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
        image: create_image_links(promo.image),
        url: promo.perma_url,
        subtitle: promo.subtitle,
        name: promo.title,
        type_field: promo.type_field,
        language: promo.language,
        list: promo.list,
        list_count: promo.list_count.map(parse_type),
        list_type: promo.list_type,
        play_count: promo.play_count.map(parse_type),
        year: promo.year.map(parse_type),
        explicit: parse_type(promo.explicit_content),
        square_image: promo.more_info.square_image,
        editorial_language: promo.more_info.editorial_language,
        position: promo.more_info.position.map(parse_type),
        release_year: promo.more_info.release_year,
    }
}
