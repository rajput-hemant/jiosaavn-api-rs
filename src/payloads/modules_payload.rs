use super::{
    album_paylaod::album_payload, chart_payload, radio_payload, song_payload, trending_payload,
};
use crate::{
    models::{
        misc::Union,
        modules::{
            ArtistRecoRequest, ArtistRecoResponse, CityModRequest, CityModRequestMultipleTune,
            CityModResponse, CityModResponseMultipleTune, DiscoverRequest, DiscoverResonse, Module,
            ModulesPlaylistRequest, ModulesPlaylistResponse, ModulesRequest, ModulesResponse,
            PromoRequest, PromoResponse, TagMixRequest, TagMixResponse,
        },
    },
    utils::{create_image_links, parse_bool, parse_explicit_content, parse_type},
};

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
    let mods = modules.modules;
    let artist_recos_mods = mods.artist_recos.unwrap_or_default();
    let city_mod_mods = mods.city_mod.unwrap_or_default();
    let mixes_mods = mods.tag_mixes.unwrap_or_default();

    ModulesResponse {
        trending: Module {
            title: mods.new_trending.title,
            subtitle: mods.new_trending.subtitle,
            position: mods.new_trending.position,
            source: "/get/trending".to_string(),
            featured_text: mods.new_trending.featured_text,
            data: trending_payload(modules.new_trending),
        },

        charts: Module {
            title: mods.charts.title,
            subtitle: mods.charts.subtitle,
            position: mods.charts.position,
            source: "/get/charts".to_string(),
            featured_text: mods.charts.featured_text,
            data: modules.charts.into_iter().map(chart_payload).collect(),
        },

        albums: Module {
            title: mods.new_albums.title,
            subtitle: mods.new_albums.subtitle,
            position: mods.new_albums.position,
            source: "/get/albums".to_string(),
            featured_text: mods.new_albums.featured_text,
            data: modules
                .new_albums
                .into_iter()
                .map(|i| match i {
                    Union::A(a) => Union::A(album_payload(a)),
                    Union::B(s) => Union::B(song_payload(s)),
                })
                .collect(),
        },

        playlists: Module {
            title: mods.top_playlists.title,
            subtitle: mods.top_playlists.subtitle,
            position: mods.top_playlists.position,
            source: "/get/featured-playlists".to_string(),
            featured_text: mods.top_playlists.featured_text,
            data: modules
                .top_playlists
                .into_iter()
                .map(module_playlist_payload)
                .collect(),
        },

        radio: Module {
            title: mods.radio.title,
            subtitle: mods.radio.subtitle,
            position: mods.radio.position,
            source: "/get/featured-stations".to_string(),
            featured_text: mods.radio.featured_text,
            data: modules.radio.into_iter().map(radio_payload).collect(),
        },

        artist_recos: Module {
            title: artist_recos_mods.title,
            subtitle: artist_recos_mods.subtitle,
            featured_text: artist_recos_mods.featured_text,
            position: artist_recos_mods.position,
            source: "artist_recos|artistRecos".to_string(),
            data: match modules.artist_recos {
                Some(artist_recos) => artist_recos.into_iter().map(artist_reco_payload).collect(),
                None => vec![],
            },
        },

        discover: Module {
            title: "".to_string(),
            subtitle: "".to_string(),
            featured_text: Some("".to_string()),
            position: 0,
            source: "discover".to_string(),
            data: modules
                .browse_discover
                .into_iter()
                .map(discover_payload)
                .collect(),
        },

        city_mod: Module {
            title: city_mod_mods.title,
            subtitle: city_mod_mods.subtitle,
            position: city_mod_mods.position,
            source: "city_mod|cityMod".to_string(),
            featured_text: city_mod_mods.featured_text,
            data: match modules.city_mod {
                Some(city_mod) => city_mod.into_iter().map(city_mod_payload).collect(),
                None => vec![],
            },
        },

        mixes: Module {
            title: mixes_mods.title,
            subtitle: mixes_mods.subtitle,
            featured_text: mixes_mods.featured_text,
            position: mixes_mods.position,
            source: "mixes".to_string(),
            data: match modules.tag_mixes {
                Some(mixes) => mixes.into_iter().map(tag_mixes_payload).collect(),
                None => vec![],
            },
        },

        promos: modules
            .promos
            .into_iter()
            .filter(|(key, _)| key.contains("promo"))
            .enumerate()
            .map(|(i, (key, promo))| {
                let module = mods.promos.get(&key).unwrap();
                (
                    format!("promo{}", i),
                    Module {
                        title: module.title.clone(),
                        subtitle: module.subtitle.clone(),
                        position: module.position,
                        source: format!("promo{}", i),
                        featured_text: module.featured_text.clone(),
                        data: promo_payload(promo),
                    },
                )
            })
            .collect(),

        global_config: modules.global_config,
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
        tags: discover.more_info.tags,
        is_featured: parse_type(discover.more_info.is_featured),
        sub_type: discover.more_info.sub_type,
        video_thumbnail: discover.more_info.video_thumbnail,
        video_url: discover.more_info.video_url,
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

fn module_playlist_payload(playlist: ModulesPlaylistRequest) -> ModulesPlaylistResponse {
    let more_info = playlist.more_info;
    ModulesPlaylistResponse {
        id: playlist.id,
        name: playlist.title,
        subtitle: playlist.subtitle,
        type_field: playlist.type_field,
        image: create_image_links(playlist.image),
        url: playlist.perma_url,
        explicit: parse_bool(playlist.explicit_content),
        user_id: more_info.uid,
        song_count: parse_type(more_info.song_count),
        firstname: more_info.firstname,
        follower_count: parse_type(more_info.follower_count),
        last_updated: more_info.last_updated,
    }
}

fn promo_payload(promo: Vec<PromoRequest>) -> Vec<PromoResponse> {
    promo
        .into_iter()
        .map(|promo| PromoResponse {
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
        })
        .collect()
}
