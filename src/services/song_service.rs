use super::api_service::http;

pub async fn get_song_details_by_id(ids: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "song.getDetails",
        false,
        Some(
            vec![("pids".to_string(), ids.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result)
}

pub async fn get_song_details_by_link(link: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "webapi.get",
        false,
        Some(
            vec![
                ("token".to_string(), link.to_string()),
                ("type".to_string(), "song".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(result)
}
