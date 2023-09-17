use axum::extract::Query;
use jiosaavn::handlers::{album_details_handler, AlbumParams};
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

    let id = hash_query.get("id").cloned();
    let token = hash_query.get("token").cloned();
    let link = hash_query.get("link").cloned();

    let raw = hash_query.get("raw").cloned();
    let camel = hash_query.get("camel").cloned();

    let params = AlbumParams {
        id,
        token,
        link,
        lang: None,
        year: None,
        raw,
        camel,
    };

    let (status, payload) = album_details_handler(Query(params)).await;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!(payload.0).to_string().into())?)
}
