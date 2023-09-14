use std::collections::HashMap;

use jiosaavn::{services::module_service::get_modules, utils::parse_bool};
use serde_json::json;
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

    let lang = hash_query
        .get("lang")
        .unwrap_or(&"hindi,english".to_string())
        .to_owned();
    let raw = hash_query
        .get("raw")
        .unwrap_or(&"false".to_string())
        .to_owned();

    let camel = hash_query
        .get("camel")
        .unwrap_or(&"false".to_string())
        .to_owned();

    let modules = json!(get_modules(lang, parse_bool(raw), parse_bool(camel)).await);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(modules.to_string().into())?)
}
