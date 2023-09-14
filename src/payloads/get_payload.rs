use super::{album_payload, playlist_payload, song_payload};
use crate::{
    models::{
        get::{
            ChartRequest, ChartResponse, RadioRequest, RadioResponse, TrendingRequest,
            TrendingResponse,
        },
        misc::Union3,
    },
    utils::{create_image_links, parse_bool},
};

pub fn trending_payload(trending: TrendingRequest) -> Vec<TrendingResponse> {
    trending
        .into_iter()
        .map(|i| match i {
            Union3::A(a) => Union3::A(album_payload(a)),
            Union3::B(s) => Union3::B(song_payload(s)),
            Union3::C(p) => Union3::C(playlist_payload(p)),
        })
        .collect()
}

pub fn chart_payload(chart: ChartRequest) -> ChartResponse {
    let more_info = chart.more_info.unwrap_or_default();

    ChartResponse {
        id: chart.id,
        image: create_image_links(chart.image),
        name: chart.title,
        type_field: chart.type_field,
        url: chart.perma_url,
        subtitle: chart.subtitle,
        explicit: chart.explicit_content.map(parse_bool),
        count: if chart.count.is_some() {
            chart.count
        } else {
            more_info.song_count
        },
        listname: chart.listname,
        language: chart.language,
        firstname: more_info.firstname,
    }
}

pub fn radio_payload(radio: RadioRequest) -> RadioResponse {
    RadioResponse {
        id: radio.id,
        name: radio.title,
        subtitle: radio.subtitle,
        type_field: radio.type_field,
        image: create_image_links(radio.image),
        url: radio.perma_url,
        explicit: parse_bool(radio.explicit_content),
        description: radio.more_info.description,
        featured_station_type: radio.more_info.featured_station_type,
        query: radio.more_info.query,
        color: radio.more_info.color,
        language: radio.more_info.language,
        station_display_text: radio.more_info.station_display_text,
    }
}
