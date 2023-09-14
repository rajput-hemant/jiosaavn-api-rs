use axum::Json;

use crate::models::response::{CResponse, Status};

pub async fn not_found_handler() -> Json<CResponse<String>> {
    Json(CResponse::new(Status::Failed, "Requested route not found, please check the documentation at https://docs-jiosaavn.netlify.app", None))
}
