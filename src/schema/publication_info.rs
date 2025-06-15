use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PublicationInfo {
    pub license: String,
    pub remastered: bool,
    pub source: String,
}

#[derive(Debug, Error)]
pub enum PublicationParsingError {
    #[error("Missing source field")]
    Source,
    #[error("Missing remastered field")]
    Remastered,
    #[error("Missing license field")]
    License,
}

impl TryFrom<&Value> for PublicationInfo {
    type Error = PublicationParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(PublicationInfo {
            license: json_utils::get_field_from_json(json, "license")
                .as_str()
                .map(String::from)
                .ok_or(PublicationParsingError::License)?,
            remastered: json_utils::get_field_from_json(json, "remaster")
                .as_bool()
                .ok_or(PublicationParsingError::Remastered)?,
            source: json_utils::get_field_from_json(json, "title")
                .as_str()
                .map(String::from)
                .ok_or(PublicationParsingError::Source)?,
        })
    }
}
