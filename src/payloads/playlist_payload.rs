use crate::{
    models::playlist::{PlaylistRequest, PlaylistResponse},
    utils::create_image_links,
};

pub fn playlist_payload(playlist: PlaylistRequest) -> PlaylistResponse {
    PlaylistResponse {
        id: playlist.listid,
        name: playlist.listname,
        url: playlist.perma_url,
        follower_count: playlist.follower_count.parse().unwrap_or_default(),
        user_id: playlist.uid,
        last_updated: playlist.last_updated.parse().unwrap_or_default(),
        username: playlist.username,
        firstname: playlist.firstname,
        lastname: playlist.lastname,
        image: create_image_links(&playlist.image),
        share: playlist.share.parse().unwrap_or_default(),
        type_field: playlist.type_field,
        list_count: playlist.list_count.parse().unwrap_or_default(),
        fan_count: playlist
            .fan_count
            .replace(",", "")
            .parse()
            .unwrap_or_default(),
        is_dolby_playlist: playlist.is_dolby_playlist,
        video_available: playlist.video_available,
        video_count: playlist.video_count,
        meta_html: playlist.meta_html,
        songs: playlist.songs,
    }
}
