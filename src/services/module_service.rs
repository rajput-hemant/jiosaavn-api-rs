use crate::models::modules::ModulesRequest;

use super::api_service::http;

pub async fn get_modules(language: &str) -> Result<ModulesRequest, reqwest::Error> {
    let result: ModulesRequest = http(
        "content.getBrowseModules",
        true,
        Some(
            vec![("language".to_string(), language.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(result)
}
