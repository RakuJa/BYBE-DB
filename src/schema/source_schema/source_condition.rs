use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::common::description::Description;
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;

pub struct SourceCondition {
    pub foundry_id: String,
    pub name: String,
    pub rule_fallback: Description,
    pub publication_info: PublicationInfo,
    pub overrides: Vec<String>,
    pub is_perpetual: bool,
    pub is_stackable: bool, // value => isvalue = true
    pub value: Option<i64>,
    pub group: Option<String>,
}

#[derive(Debug, Error)]
pub enum SourceConditionParsingError {
    #[error("Source id missing")]
    IdMissing,
    #[error("Mandatory Name field is missing from json")]
    NameFieldMissing,
    #[error("Publication info could not be parsed")]
    PublicationError(#[from] PublicationParsingError),
}

impl TryFrom<&Value> for SourceCondition {
    type Error = SourceConditionParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let foundry_id = get_field_from_json(json, "_id")
            .as_str()
            .map(String::from)
            .ok_or(SourceConditionParsingError::IdMissing)?;
        let system_json = get_field_from_json(json, "system");
        let publication_json = get_field_from_json(&system_json, "publication");
        let duration_json = get_field_from_json(&system_json, "duration");
        let value_json = get_field_from_json(&system_json, "value");
        let description_json = get_field_from_json(&system_json, "description");
        let name = get_field_from_json(json, "name")
            .as_str()
            .map(|x| x.to_string())
            .ok_or(SourceConditionParsingError::NameFieldMissing)?;

        Ok(SourceCondition {
            name,
            foundry_id,
            publication_info: PublicationInfo::try_from(&publication_json)?,
            overrides: get_field_from_json(&system_json, "overrides")
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_lowercase())
                .collect(),
            is_perpetual: get_field_from_json(&duration_json, "perpetual")
                .as_bool()
                .unwrap_or(false),
            is_stackable: get_field_from_json(&value_json, "isValued")
                .as_bool()
                .unwrap(),
            value: get_field_from_json(&value_json, "value").as_i64(),
            group: get_field_from_json(&system_json, "group")
                .as_str()
                .map(|x| x.to_string()),
            rule_fallback: Description::from(
                get_field_from_json(&description_json, "value")
                    .as_str()
                    .unwrap()
                    .to_string(),
            ),
        })
    }
}
