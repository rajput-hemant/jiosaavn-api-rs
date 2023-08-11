use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Lyrics {
    Success(LyricsSuccess),
    Error(LyricsError),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct LyricsSuccess {
    pub lyrics: String,
    pub lyrics_copyright: String,
    pub snippet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LyricsError {
    pub error: ErrorMsg,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMsg {
    pub msg: String,
}
