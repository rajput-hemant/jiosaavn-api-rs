use axum::extract::Query;
use jiosaavn::handlers::{recommend_artists_songs_handler, ArtistParams};
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

    let artist_id = hash_query.get("artist_id").cloned();
    let song_id = hash_query.get("song_id").cloned();
    let lang = hash_query.get("lang").cloned();
    let page = hash_query.get("page").cloned();
    let cat = hash_query.get("cat").cloned();
    let sort = hash_query.get("sort").cloned();

    let raw = hash_query.get("raw").cloned();
    let camel = hash_query.get("camel").cloned();

    let params = ArtistParams {
        id: None,
        link: None,
        token: None,
        song_id,
        artist_id,
        page,
        n_song: None,
        n_album: None,
        cat,
        sort,
        lang,
        raw,
        camel,
    };
    let (status, payload) = recommend_artists_songs_handler(Query(params)).await;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!(payload.0).to_string().into())?)
}
