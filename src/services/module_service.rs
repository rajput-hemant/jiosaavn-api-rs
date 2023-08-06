use crate::{models::modules::ModulesResponse, payloads::modules_paylod};

use super::api_service::http;

/// Helper function to make request to `content.getBrowseModules` endpoint of JioSaavn API to get home modules / launch data
/// and return modules payload
///
/// ## Arguments
///
/// * `languages` - Comma separated languages
/// * Available languages: `hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,
///  `gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`,
///  `rajasthani`, `odia`, `assamese`
///
/// ## Returns
///
/// * `Result<ModulesResponse, reqwest::Error>` - Result of modules payload
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_modules() -> Result<(), reqwest::Error> {
        let result = get_modules("hindi,english").await?;

        dbg!("{:?}", result);

        Ok(())
    }
}
