use axum::extract::{Path, Query};
use jiosaavn::handlers::{artist_songs_albums_handler, ArtistParams};
use serde_json::json;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();

    let path = parsed_url.path().split('/').last().unwrap().to_string();

    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    let id = hash_query.get("id").cloned();
    let page = hash_query.get("page").cloned();
    let cat = hash_query.get("cat").cloned();
    let sort = hash_query.get("sort").cloned();

    let raw = hash_query.get("raw").cloned();
    let camel = hash_query.get("camel").cloned();

    let params = ArtistParams {
        id,
        link: None,
        token: None,
        song_id: None,
        artist_id: None,
        page,
        n_song: None,
        n_album: None,
        cat,
        sort,
        lang: None,
        raw,
        camel,
    };

    let (status, payload) = artist_songs_albums_handler(Query(params), Path(path)).await;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!(payload.0).to_string().into())?)
}
