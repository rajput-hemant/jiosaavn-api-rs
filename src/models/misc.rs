use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rights {
    pub code: Value,
    pub cacheable: Value,
    pub delete_cached_object: Option<Value>,
    pub reason: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Union<A, B> {
    A(A),
    B(B),
}
