use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rights {
    pub code: Value,
    pub cacheable: Value,
    pub delete_cached_object: Value,
    pub reason: Value,
}
