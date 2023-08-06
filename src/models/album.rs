use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistMap {
    pub primary_artists: Vec<AlbumArtistRequest>,
    pub featured_artists: Vec<AlbumArtistRequest>,
    pub artists: Vec<AlbumArtistRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumArtistRequest {
    pub id: String,
    pub name: String,
    pub role: String,
    pub image: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub perma_url: String,
}
