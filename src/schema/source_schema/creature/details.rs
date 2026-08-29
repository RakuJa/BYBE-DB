use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::common::description::Description;
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub struct RawDetails {
    pub languages_details: String,
    pub languages: Vec<String>,
    pub level: i64, //i8,
    pub publication_info: PublicationInfo,
    pub description: String,
    pub blurb: String,
}

#[derive(Debug, Error)]
pub enum DetailsParsingError {
    #[error("Level field missing")]
    LevelMissing,
    #[error("Level value is NaN")]
    LevelNaN,
    #[error("Mandatory language details field is missing from json")]
    LanguageDetailsFieldMissing,
    #[error("Mandatory language field is missing from json")]
    LanguageFieldMissing,
    #[error("Publication info could not be parsed")]
    PublicationError(#[from] PublicationParsingError),
}
impl TryFrom<&Value> for RawDetails {
    type Error = DetailsParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let languages_json = &json_utils::get_field_from_json(json, "languages");
        let publication_json = &json_utils::get_field_from_json(json, "publication");
        let level = json_utils::get_field_from_json(json, "level")
            .get("value")
            .ok_or(DetailsParsingError::LevelMissing)?
            .as_i64()
            .ok_or(DetailsParsingError::LevelNaN)?;
        Ok(RawDetails {
            level,
            languages_details: json_utils::get_field_from_json(languages_json, "details")
                .as_str()
                .map(String::from)
                .ok_or(DetailsParsingError::LanguageDetailsFieldMissing)?,
            languages: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(languages_json, "value")
                    .as_array()
                    .ok_or(DetailsParsingError::LanguageFieldMissing)?,
            ),
            publication_info: PublicationInfo::try_from(publication_json)?,
            description: json_utils::get_field_from_json(json, "publicNotes")
                .as_str()
                .map(Description::from)
                .map(|x| x.resolve(Some(level)))
                .unwrap_or_default(),
            blurb: json_utils::get_field_from_json(json, "blurb")
                .as_str()
                .map(Description::from)
                .map(|x| x.resolve(Some(level)))
                .unwrap_or_default(),
        })
    }
}
