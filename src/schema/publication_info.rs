use crate::schema::json_utils;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct PublicationInfo {
    pub license: String,
    pub remastered: bool,
    pub source: String,
}

impl PublicationInfo {
    pub fn init_from_json(json: &Value) -> PublicationInfo {
        PublicationInfo {
            license: json_utils::get_field_from_json(json, "license")
                .as_str()
                .unwrap()
                .to_string(),
            remastered: json_utils::get_field_from_json(json, "remaster")
                .as_bool()
                .unwrap(),
            source: json_utils::get_field_from_json(json, "title")
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
