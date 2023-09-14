use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Quality {
    // Bool(bool),
    String(String),
    List(Vec<QualityObject>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityObject {
    pub quality: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Union3<A, B, C> {
    A(A),
    B(B),
    C(C),
}
