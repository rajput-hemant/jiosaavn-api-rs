use base64::{engine::general_purpose, Engine as _};
use openssl::symm::{decrypt, Cipher};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value, Value};
use std::str::FromStr;

use crate::models::misc::{Quality, QualityObject, Union};

/// A utility function for creating download links of different qualities
///
/// ## Arguments
///
/// * `encrypted_media_url` - encrypted media url from the API
///
/// ## Returns
///
/// * `Quality` - An enum that holds the download link(s) with different qualities
pub fn create_download_links(encrypted_media_url: String) -> Quality {
    let qualities = vec![
        ("_12", "12kbps"),
        ("_48", "48kbps"),
        ("_96", "96kbps"),
        ("_160", "160kbps"),
        ("_320", "320kbps"),
    ];

    let key = b"38346591";

    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_media_url.as_bytes())
        .unwrap();

    let cipher = Cipher::des_ecb();
    let dechipher = decrypt(cipher, key, None, &encrypted_data);

    if dechipher.is_err() {
        return Quality::String(encrypted_media_url);
    }

    let decrypted_link = String::from_utf8(dechipher.unwrap()).unwrap();

    let download_links = qualities
        .into_iter()
        .map(|quality| QualityObject {
            quality: quality.1.to_string(),
            link: decrypted_link.replace("_96", quality.0),
        })
        .collect();

    Quality::List(download_links)
}

/// Utility function for creating image links of different qualities
///
/// ## Arguments
///
/// * `link` - image link from the API
///
/// ## Returns
///
/// * `Quality` - An enum that holds the image link(s) with different qualities
pub fn create_image_links(link: String) -> Quality {
    let qualities = vec!["50x50", "150x150", "500x500"];

    if let Some(q) = qualities.iter().find(|&&q| link.contains(q)) {
        let download_links = qualities
            .iter()
            .map(|&quality| QualityObject {
                quality: quality.to_string(),
                link: link.replace(q, quality),
            })
            .collect();

        return Quality::List(download_links);
    }

    Quality::String(link)
}

pub fn is_jio_saavn_link(url: &str) -> bool {
    let regex = regex::Regex::new(
        r"^(https?:\/\/)?(www.)?jiosaavn\.com\/(song|shows|album|artist|featured)\/(.+)$",
    )
    .unwrap();

    regex.is_match(url.trim())
}

/// A utility function for parsing explicit content string to boolean
///
/// ## Arguments
///
/// * `v` - explicit content string
///
/// ## Returns
///
/// * `bool` - A boolean value that indicates if the content is explicit or not
pub fn parse_explicit_content(v: String) -> bool {
    matches!(v.as_str(), "1" | "true")
}

/// A utility function for extracting the token from a link
///
/// ## Arguments
///
/// * `link` - link from the `jiosaavn.com` domain
///
/// ## Returns
///
/// * `String` - extracted token
pub fn token_from_link(link: String) -> String {
    link.split('/').last().unwrap().to_string()
}

/// A utility function for parsing string to a generic type
///
/// ## Arguments
///
/// * `from` - string to parse
///
/// ## Returns
///
/// * `T` - parsed type
pub fn parse_type<T: FromStr + Default>(from: String) -> T {
    from.parse().unwrap_or_default()
}

/// Utility function to validate the `langs` query
///
/// ## Arguments
///
/// * `langs` - Comma separated languages
///
/// ## Returns
///
/// * `String` - Comma separated valid languages
pub fn valid_langs(langs: String) -> String {
    let valid_langs = [
        "hindi",
        "english",
        "punjabi",
        "tamil",
        "telugu",
        "marathi",
        "gujarati",
        "bengali",
        "kannada",
        "bhojpuri",
        "malayalam",
        "urdu",
        "haryanvi",
        "rajasthani",
        "odia",
        "assamese",
    ];

    let filtered_langs = langs
        .split(',')
        .filter(|l| valid_langs.contains(&l.trim()))
        .collect::<Vec<_>>()
        .join(",");

    if filtered_langs.is_empty() {
        "hindi,english".to_string()
    } else {
        filtered_langs
    }
}

/// A utility function for parsing string to boolean
///
/// ## Arguments
///
/// * `v` - string
///
/// ## Returns
///
/// * `bool` - A boolean value
pub fn parse_bool(from: String) -> bool {
    matches!(from.as_str(), "1" | "true")
}

/// A utility function to format the payload in camel case if `camel` is true
///
/// ## Arguments
///
/// * `value` - payload
/// * `camel` - boolean value to indicate if the payload should be formatted in camel case
/// * `formatter` - function to format the payload
///
/// ## Returns
pub fn formatted_payload<F, T>(
    value: Value,
    camel: bool,
    formatter: &dyn Fn(F) -> T,
) -> Union<Value, T>
where
    F: DeserializeOwned,
    T: DeserializeOwned + Serialize,
{
    from_value(value)
        .map(|payload| match camel {
            true => Union::A(to_camel_case(to_value(formatter(payload)).unwrap())),
            false => Union::B(formatter(payload)),
        })
        .unwrap()
}

fn to_camel_case(input: Value) -> Value {
    match input {
        Value::Object(o) => Value::Object(
            o.into_iter()
                .map(|(k, v)| (convert_key_to_camel_case(&k), to_camel_case(v)))
                .collect(),
        ),

        Value::Array(vec) => Value::Array(vec.into_iter().map(to_camel_case).collect()),

        other => from_value(other).unwrap(),
    }
}

fn convert_key_to_camel_case(key: &str) -> String {
    key.split('_')
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_string()
            } else {
                let mut chars = s.chars();
                match chars.next() {
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                    None => String::new(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join("")
}
