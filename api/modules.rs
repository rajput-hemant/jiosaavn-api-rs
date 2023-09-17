use std::collections::HashMap;

use axum::extract::Query;
use jiosaavn::handlers::{modules_handler, ModulesParams};
use serde_json::json;
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

    let lang = hash_query.get("lang").cloned();
    let raw = hash_query.get("raw").cloned();
    let camel = hash_query.get("camel").cloned();

    let params = ModulesParams { lang, raw, camel };
    let (status, payload) = modules_handler(Query(params)).await;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!(payload.0).to_string().into())?)
}
