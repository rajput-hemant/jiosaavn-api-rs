use axum::Json;

use crate::config::DOCS_URL;
use crate::models::response::{CResponse, Status};

/// Handler for `404` route
pub async fn not_found_handler() -> Json<CResponse<String>> {
    Json(CResponse::new(
        Status::Failed,
        format!("Requested route not found, please check the documentation at {DOCS_URL}"),
        None,
    ))
}
