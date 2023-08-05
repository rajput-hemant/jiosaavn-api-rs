use super::api_service::http;

pub async fn get_album_details_by_id(id: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "content.getAlbumDetails",
        false,
        Some(
            vec![("albumid".to_string(), id.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result)
}

pub async fn get_album_details_by_link(link: &str) -> Result<serde_json::Value, reqwest::Error> {
    let result: serde_json::Value = http(
        "webapi.get",
        false,
        Some(
            vec![
                ("token".to_string(), link.to_string()),
                ("type".to_string(), "album".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
    )
    .await?;

    Ok(result)
}
