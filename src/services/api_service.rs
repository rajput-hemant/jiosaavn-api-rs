use reqwest::{
    header::{HeaderMap, COOKIE},
    Client, ClientBuilder, Error, Url,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

const BASE_URL: &str = "https://www.jiosaavn.com/api.php";

/// ApiService struct
pub struct ApiService {
    client: Client,
    base_url: Url,
}

impl ApiService {
    fn new() -> Result<Self, Error> {
        let mut base_url = Url::parse(BASE_URL).unwrap();

        base_url
            .query_pairs_mut()
            .append_pair("_format", "json")
            .append_pair("_marker", "0")
            .append_pair("ctx", "web6dot0");

        let client = ClientBuilder::new().build()?;

        Ok(Self { client, base_url })
    }
}

type Query = HashMap<String, String>;

/// Main helper function to make request to JioSaavn API
///
/// ## Arguments
///
/// * `path` - API endpoint path
/// * `is_version_4` - Whether to use version 4 of JioSaavn API
/// * `query` - Query parameters
///
/// ## Returns
///
/// * `Result<T, Error>` - Result of API response
pub async fn http<T: DeserializeOwned>(
    path: &str,
    is_version_4: bool,
    query: Option<Query>,
) -> Result<T, Error> {
    let api_service = ApiService::new()?;
    let mut url = api_service.base_url;

    url.query_pairs_mut().append_pair("__call", path);

    if is_version_4 {
        url.query_pairs_mut().append_pair("api_version", "4");
    }

    if let Some(query_params) = query {
        let mut query_pairs = url.query_pairs_mut();

        query_params.iter().for_each(|(key, value)| {
            query_pairs.append_pair(key, value);
        });
    }

    let language_header = url
        .query_pairs()
        .find(|(key, _)| key == "language")
        .map_or("english".to_string(), |(_, value)| value.into_owned());

    let cookie_value = format!("L={language_header}; gdpr_acceptance=true; DL=english");

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_value.parse().unwrap());

    let body: T = api_service
        .client
        .get(url.clone())
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(body)
}
