use crate::{
    config::LAUNCH_DATA,
    models::{
        misc::Union,
        modules::RModules,
        response::{CResponse, Status},
    },
    payloads::modules_payload,
};

use super::api_service::http;

/// Helper function to make request to `content.getBrowseModules` endpoint of JioSaavn API,
/// to get home modules / launch data and return modules payload
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
/// * `RModules` - modules payload
pub async fn get_modules(lang: String, raw: bool, _: bool) -> RModules {
    let response = http(
        LAUNCH_DATA,
        true,
        Some(vec![("language".to_string(), lang)].into_iter().collect()),
    )
    .await;

    match response {
        Ok(modules) => {
            if raw {
                Union::A(modules)
            } else {
                // TODO: Add camel case conversion

                Union::B(CResponse::new(
                    Status::Success,
                    "✅ Home Data fetched successfully",
                    Some(modules_payload(modules)),
                ))
            }
        }
        Err(e) => {
            println!("Error: {e}");

            Union::B(CResponse::new(
                Status::Failed,
                "❌ Something went wrong",
                None,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_modules() {
        let modules = get_modules("hindi,english".to_string(), false, false).await;

        dbg!("{:?}", modules);
    }

    #[tokio::test]
    async fn test_get_modules_raw() {
        let modules = get_modules("hindi,english".to_string(), true, false).await;

        dbg!("{:?}", modules);
    }
}
