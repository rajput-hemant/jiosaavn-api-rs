use crate::{
    config::LAUNCH_DATA,
    models::{
        misc::Union,
        modules::RModules,
        response::{CResponse, Status},
    },
    payloads::modules_payload,
    utils::{formatted_payload, valid_langs},
};

use super::api_service::http;

/// Helper function to get home launch data/modules from JioSaavn API
///
/// ## Arguments
///
/// * `languages` - Comma separated languages
/// * Available languages: `hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,
///  `gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`,
///  `rajasthani`, `odia`, `assamese`
/// * `raw` - Whether to return raw response or not
/// * `camel` - Whether to convert response to camel case or not
///
/// ## Returns
///
/// * `Result<RModules, String>` - Modules payload
pub async fn get_modules(langs: String, raw: bool, camel: bool) -> Result<RModules, String> {
    let response = http(
        LAUNCH_DATA,
        true,
        Some(
            vec![("language".to_string(), valid_langs(langs))]
                .into_iter()
                .collect(),
        ),
    )
    .await;

    match response {
        Ok(modules) => {
            if raw {
                Ok(Union::A(modules))
            } else {
                let payload = formatted_payload(modules, camel, &modules_payload);

                Ok(Union::B(CResponse::new(
                    Status::Success,
                    "✅ Home Data fetched successfully".to_string(),
                    Some(payload),
                )))
            }
        }
        Err(e) => {
            println!("Error: {e}");

            Err("❌ Something went wrong".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_modules() {
        let modules = get_modules("hindi,english".to_string(), false, false).await;

        assert!(modules.is_ok());

        match modules.unwrap() {
            Union::A(_) => {}
            Union::B(res) => {
                assert_eq!(res.status, Status::Success);
                assert!(res.data.is_some());

                match res.data.unwrap() {
                    Union::A(_) => {}
                    Union::B(mods) => {
                        assert!(!mods.albums.data.is_empty());
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_modules_raw() {
        let modules = get_modules("hindi,english".to_string(), true, false).await;

        assert!(modules.is_ok());

        match modules.unwrap() {
            Union::A(modules) => {
                assert!(!modules["new_albums"].as_array().unwrap().is_empty());
            }
            Union::B(_) => {}
        }
    }

    #[tokio::test]
    async fn test_get_modules_camel() {
        let modules = get_modules("hindi,english".to_string(), false, true).await;

        assert!(modules.is_ok());

        match modules.unwrap() {
            Union::A(res) => {
                assert_eq!(res["status"], "Success");
                assert!(res["data"].is_object());
                assert!(res["data"]["cityMod"].is_object());
                assert!(!res["data"]["cityMod"]["data"]
                    .as_array()
                    .unwrap()
                    .is_empty());
            }
            Union::B(_) => {}
        }
    }
}
