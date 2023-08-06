use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rights {
    pub code: String,
    pub cacheable: String,
    pub delete_cached_object: Option<String>,
    pub reason: Option<String>,
}
