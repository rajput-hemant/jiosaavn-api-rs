use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub ctr: f64,
    pub entity: f64,
    pub image: String,
    pub role: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mini_obj: bool,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: bool,
    pub is_followed: bool,
}

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
