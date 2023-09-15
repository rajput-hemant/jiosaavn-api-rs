use axum::extract::Query;
use jiosaavn::handlers::{albums_from_same_year_handler, AlbumParams};
use serde_json::json;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

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

    let year = hash_query.get("year").cloned();
    let lang = hash_query.get("lang").cloned();
    let raw = hash_query.get("raw").cloned();
    let camel = hash_query.get("camel").cloned();

    let params = AlbumParams {
        id: None,
        token: None,
        link: None,
        lang,
        year,
        raw,
        camel,
    };

    let payload = json!(albums_from_same_year_handler(Query(params)).await.0);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(payload.to_string().into())?)
}
