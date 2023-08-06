use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Quality {
    Bool(bool),
    String(String),
    List(Vec<QualityObject>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QualityObject {
    pub quality: String,
    pub link: String,
}
