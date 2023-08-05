use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AlbumRequest {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub header_desc: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: String,
    pub play_count: String,
    pub explicit_content: String,
    pub list_count: String,
    pub list_type: String,
    pub list: String,
    pub more_info: AlbumRequestMoreInfo,
    pub year: String,
    // pub release_date: String,
    // pub primary_artists: String,
    // pub primary_artists_id: String,
    // pub albumid: String,
    pub perma_url: String,
    pub image: String,
    // pub songs: Vec<SongRequest>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumRequestMoreInfo {
    pub release_date: String,
    // pub song_count: String,
    #[serde(rename = "artistMap")]
    pub artist_map: ArtistMap,
}

#[derive(Debug, Deserialize)]
pub struct ArtistMap {
    pub primary_artists: Vec<AlbumArtistRequest>,
    pub featured_artists: Vec<AlbumArtistRequest>,
    pub artists: Vec<AlbumArtistRequest>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumArtistRequest {
    pub id: String,
    pub name: String,
    pub role: String,
    pub image: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
}
