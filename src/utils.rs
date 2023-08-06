use crate::models::quality::{Quality, QualityObject};

pub fn create_image_links(link: &str) -> Quality {
    let qualities = vec!["50x50", "150x150", "500x500"];

    if qualities.iter().all(|quality| !link.contains(quality)) {
        return Quality::String(link.to_string());
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

pub fn parse_explicit_content(v: String) -> bool {
    match v.as_str() {
        "1" | "true" => true,
        _ => false,
    }
}
