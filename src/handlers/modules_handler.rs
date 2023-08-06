use axum::{extract::Query, Json};

use crate::{
    models::{
        modules::ModulesResponse,
        response::{CustomResponse, StatusCode},
    },
    services::module_service::get_modules,
};

pub async fn modules_handler(
    language: Option<Query<String>>,
) -> Json<CustomResponse<ModulesResponse>> {
    let modules = get_modules(
        &language
            .map(|lang| lang.0)
            .unwrap_or_else(|| "hindi,english".to_string()),
    )
    .await;

    Json(CustomResponse::new(
        if modules.is_ok() {
            StatusCode::Success
        } else {
            StatusCode::Failed
        },
        if modules.is_ok() {
            "✅ Successfully fetched home data"
        } else {
            "❌ Failed to fetch home data"
        },
        modules.ok(),
    ))
}
