use crate::{models::modules::RModules, services::module_service::get_modules, utils::parse_bool};
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    lang: Option<String>,
    raw: Option<String>,
    camel: Option<String>,
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
///
/// ## Returns
///
/// * `Json<RModules>` - Json response
///
pub async fn modules_handler(Query(params): Query<Params>) -> Json<RModules> {
    let languages = params.lang.unwrap_or_else(|| "hindi,english".to_string());
    let raw = parse_bool(params.raw.unwrap_or_else(|| "false".to_string()));
    let camel = parse_bool(params.camel.unwrap_or_else(|| "false".to_string()));

    Json(get_modules(languages, raw, camel).await)
}
