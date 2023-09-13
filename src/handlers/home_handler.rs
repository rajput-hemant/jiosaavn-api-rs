use axum::response::Html;

use crate::services::home_service::home_service;

pub async fn home_handler() -> Html<String> {
    home_service()
}
