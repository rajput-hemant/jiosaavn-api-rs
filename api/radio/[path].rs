use axum::extract::{Path, Query};
use jiosaavn::{
    handlers::{create_radio_handler, RadioParams},
    models::radio::RadioStationType,
};
use serde_json::json;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let hash_query: HashMap<String, String> = Url::parse(&req.uri().to_string())
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect();

    let path = match hash_query.get("path").cloned().unwrap().as_str() {
        "featured" => RadioStationType::Featured,
        "artist" => RadioStationType::Artist,
        "entity" => RadioStationType::Entity,
        _ => unreachable!(),
    };

    let song_id = hash_query.get("song_id").cloned();
    let artist_id = hash_query.get("artist_id").cloned();
    let name = hash_query.get("name").cloned();
    let n = hash_query.get("n").cloned();
    let mode = hash_query.get("mode").cloned();
    let lang = hash_query.get("lang").cloned();
    let id = hash_query.get("id").cloned();
    let entity_type = hash_query.get("type").cloned();

    let raw = hash_query.get("raw").cloned();
    let camel = hash_query.get("camel").cloned();

    let params = RadioParams {
        song_id,
        artist_id,
        name,
        n,
        mode,
        lang,
        id,
        entity_type,
        raw,
        camel,
    };

    let (status, payload) = create_radio_handler(Query(params), Path(path)).await;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!(payload.0).to_string().into())?)
}
