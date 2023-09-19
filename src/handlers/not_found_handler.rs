use axum::{http::StatusCode, Json};

use crate::config::DOCS_URL;
use crate::models::response::{CResponse, Status};

/// Handler for `404` route
pub async fn not_found_handler() -> (StatusCode, Json<CResponse<String>>) {
    (
        StatusCode::NOT_FOUND,
        Json(CResponse::new(
            Status::Failed,
            format!("Requested route not found, please check the documentation at {DOCS_URL}"),
            None,
        )),
    )
}
