use crate::{models::modules::ModulesResponse, payloads::modules_playlaod::modules_paylod};

use super::api_service::http;

pub async fn get_modules(languages: &str) -> Result<ModulesResponse, reqwest::Error> {
    let result = http(
        "content.getBrowseModules", // `webapi.getLaunchData`
        true,
        Some(
            vec![("language".to_string(), languages.to_string())]
                .into_iter()
                .collect(),
        ),
    )
    .await?;

    Ok(modules_paylod(result))
}
