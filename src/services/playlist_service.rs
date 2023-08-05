use super::api_service::http;

pub async fn get_playlist_details_by_id(id: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "playlist.getDetails",
        false,
        Some(
            vec![("listid".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result)
}
