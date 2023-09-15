use base64::{engine::general_purpose, Engine as _};
use openssl::symm::{decrypt, Cipher};
use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::models::misc::{Quality, QualityObject};

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
        .decode(&encrypted_media_url.as_bytes())
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
            link: decrypted_link.replace("_96", &format!("{}", quality.0)),
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

    if qualities.iter().all(|&quality| !link.contains(quality)) {
        return Quality::String(link);
    }

    let mut image_links = Vec::new();

    for quality in qualities {
        let link = if link.contains("150x150") {
            link.replace("150x150", quality)
        } else {
            link.replace("50x50", quality)
        };

        image_links.push(QualityObject {
            quality: quality.to_string(),
            link,
        });
    }

    Quality::List(image_links)
}

pub fn is_jio_saavn_link(url: String) -> bool {
    let regex = regex::Regex::new(
        r"^(https?:\/\/)?(www.)?jiosaavn\.com\/(song|shows|album|artist|featured)\/(.+)$",
    )
    .unwrap();

    regex.is_match(&url)
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
    match v.as_str() {
        "1" | "true" => true,
        _ => false,
    }
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
    link.split("/").last().unwrap().to_string()
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
    match from.as_str() {
        "1" | "true" => true,
        _ => false,
    }
}

use serde_json::{Map, Value};

pub fn to_camel_case<T: DeserializeOwned>(input: Value) -> T {
    match input {
        Value::Object(obj) => {
            let mut new_obj = Map::new();
            for (key, value) in obj {
                let new_key = convert_key_to_camel_case(&key);
                let new_value = to_camel_case(value);
                new_obj.insert(new_key, new_value);
            }
            // Value::Object(new_obj)
            serde_json::from_value(Value::Object(new_obj)).unwrap()
        }
        Value::Array(vec) => {
            let new_vec: Vec<Value> = vec.into_iter().map(|v| to_camel_case(v)).collect();
            // Value::Array(new_vec)
            serde_json::from_value(Value::Array(new_vec)).unwrap()
        }
        other => serde_json::from_value(other).unwrap(),
    }
}

fn convert_key_to_camel_case(key: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in key.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
