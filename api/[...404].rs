use jiosaavn::handlers::not_found_handler;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_: Request) -> Result<Response<Body>, Error> {
    let (status, payload) = not_found_handler().await;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!(payload.0).to_string().into())?)
}
