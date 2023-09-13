#![allow(dead_code)]

/* Urls */
pub const BASE_URL: &str = "https://www.jiosaavn.com/api.php";
pub const DOCS_URL: &str = "https://docs-jiosaavn.netlify.app";
pub const SITE_URL: &str = "https://jiosaavn-api-rs.vercel.app";

/* Endpoints */

// Modules
pub const LAUNCH_DATA: &str = "webapi.getLaunchData";
pub const BROWSE_MODULES: &str = "content.getBrowseModules";

//'s Song Details
pub const SONG_BY_ID: &str = "song.getDetails";
pub const SONG_BY_LINK: &str = "webapi.get";
pub const RECOMMEND_SONGS: &str = "reco.getreco";

// Album's Details
pub const ALBUM_BY_ID: &str = "content.getAlbumDetails";
pub const ALBUM_BY_LINK: &str = "webapi.get";
pub const RECOMMEND_ALBUMS: &str = "reco.getAlbumReco";
pub const ALBUM_BY_SAME_YEAR: &str = "search.topAlbumsoftheYear";

// Playlist's Details
pub const PLAYLIST_BY_ID: &str = "playlist.getDetails";
pub const PLAYLIST_BY_LINK: &str = "webapi.get";
pub const RECOMMEND_PLAYLISTS: &str = "reco.getPlaylistReco";

// Artists's Details
pub const ARTIST_BY_ID: &str = "artist.getArtistPageDetails";
pub const ARTIST_BY_LINK: &str = "webapi.get";
pub const ARTISTS_SONGS: &str = "artist.getArtistMoreSong";
pub const ARTISTS_ALBUMS: &str = "artist.getArtistMoreAlbum";
pub const ARTISTS_TOP_SONGS: &str = "search.artistOtherTopSongs";

// Search
pub const TOP_SEARCH: &str = "content.getTopSearches";
pub const SEARCH_ALL: &str = "autocomplete.get";
pub const SEARCH_SONGS: &str = "search.getResults";
pub const SEARCH_ALBUMS: &str = "search.getAlbumResults";
pub const SEARCH_ARTISTS: &str = "search.getArtistResults";
pub const SEARCH_PLAYLISTS: &str = "search.getPlaylistResults";
pub const SEARCH_MORE: &str = "search.getMoreResults";

// Radio
pub const CREATE_FEATURED_RADIO: &str = "webradio.createFeaturedStation";
pub const CREATE_ARTIST_RADIO: &str = "webradio.createArtistStation";
pub const CREATE_ENTITY_RADIO: &str = "webradio.createEntityStation";
pub const RADIO_SONGS: &str = "webradio.getSong";

// Get
pub const GET_TRENDING: &str = "content.getTrending";
pub const GET_FEATURED_PLAYLISTS: &str = "content.getFeaturedPlaylists";
pub const GET_CHARTS: &str = "content.getCharts";
pub const GET_TOP_SHOWS: &str = "content.getTopShows";
pub const GET_TOP_ARTISTS: &str = "social.getTopArtists";
pub const GET_TOP_ALBUMS: &str = "content.getAlbums";
pub const GET_FOOTER_DETAILS: &str = "webapi.getFooterDetails";
pub const GET_FEATURED_STATIONS: &str = "webradio.getFeaturedStations";
pub const GET_ACTOR_TOP_SONGS: &str = "search.actorOtherTopSongs";
pub const GET_LYRICS: &str = "lyrics.getLyrics";
