pub mod album_handler;
pub mod artist_handler;
pub mod home_handler;
pub mod lyrics_handler;
pub mod modules_handler;
pub mod not_found_handler;
pub mod playlist_handler;
pub mod radio_handler;
pub mod search_handler;
pub mod song_handler;

pub use album_handler::*;
pub use artist_handler::*;
pub use home_handler::*;
pub use lyrics_handler::lyrics_handler;
pub use modules_handler::*;
pub use not_found_handler::*;
pub use playlist_handler::*;
pub use radio_handler::*;
pub use search_handler::{
    albums_search_handler, artists_search_handler, playlists_search_handler, search_all_handler,
    songs_search_handler, top_searches_handler,
};
pub use song_handler::*;
