pub mod album_handler;
pub mod artist_handler;
pub mod modules_handler;
pub mod playlist_handler;
pub mod song_handler;

pub use album_handler::{album_details_handler, recommend_albums_handler};
pub use artist_handler::{
    artist_albums_handler, artist_details_handler, artist_songs_handler,
    recommend_artists_songs_handler,
};
pub use modules_handler::modules_handler;
pub use playlist_handler::playlist_details_handler;
pub use song_handler::{recommend_songs_handler, song_details_handler};
