use crate::{
    models::{misc::Union, modules::RModules, response::CResponse},
    services::module_service::get_modules,
    utils::{parse_bool, valid_langs},
};
use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModulesParams {
    pub lang: Option<String>,
    pub raw: Option<String>,
    pub camel: Option<String>,
}
/// Handler for `/modules` route
///
/// ## Arguments
///
/// * `language` - Optional query parameter for Comma separated languages
/// * Default languages: `hindi,english`
/// * Available languages: `hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,
///  `gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`,
///  `rajasthani`, `odia`, `assamese`
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `(StatusCode, Json<CResponse<RModules>>)` - Status code and JSON response
pub async fn modules_handler(Query(params): Query<ModulesParams>) -> (StatusCode, Json<RModules>) {
    let langs = params.lang.unwrap_or_default();
    let raw = parse_bool(params.raw.unwrap_or_default());
    let camel = parse_bool(params.camel.unwrap_or_default());

    match get_modules(valid_langs(langs), raw, camel).await {
        Ok(modules) => (StatusCode::OK, Json(modules)),

        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Union::B(CResponse::new(
                crate::models::response::Status::Failed,
                e.to_string(),
                None,
            ))),
        ),
    }
}
