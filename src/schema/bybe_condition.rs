use crate::game_system_handler::current_game_system;
use crate::schema::localize_loader;
use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::source_condition::{
    SourceCondition, SourceConditionParsingError,
};
use crate::utils::game_system_enum::GameSystem;
use crate::utils::json_utils::get_field_from_json;
use anyhow::anyhow;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BybeConditionParsingError {
    #[error("Unsupported item type")]
    UnsupportedConditionType,
    #[error("Missing item type")]
    MissingConditionType,
    #[error("Source item could not be parsed")]
    SourceConditionError(#[from] SourceConditionParsingError),
}
impl TryFrom<&Value> for BybeCondition {
    type Error = BybeConditionParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .map(|x| x.to_ascii_lowercase())
            .ok_or(BybeConditionParsingError::MissingConditionType)?;
        if !item_type.eq("condition") {
            return Err(BybeConditionParsingError::UnsupportedConditionType);
        }
        Ok(Self::from(SourceCondition::try_from(json)?))
    }
}

impl From<SourceCondition> for BybeCondition {
    fn from(condition: SourceCondition) -> Self {
        let name = condition.name.to_lowercase();
        Self {
            rule: get_condition_field(&name, "rule")
                .map(Description::from)
                .unwrap_or(condition.rule_fallback),
            note: get_condition_field(&name, "note")
                .ok()
                .map(Description::from),
            summary: get_condition_field(&name, "summary")
                .ok()
                .map(Description::from),
            publication_info: condition.publication_info,
            overrides: condition.overrides,
            is_perpetual: condition.is_perpetual,
            is_stackable: condition.is_stackable,
            value: condition.value,
            group: condition.group,
            name,
        }
    }
}

fn get_condition_field(name: &str, field: &str) -> anyhow::Result<String> {
    let l_name = name.to_lowercase();
    let l_field = field.to_lowercase();
    let dot_path = format!("PF2E.condition.{l_name}.{l_field}");

    let sf2e_result = matches!(current_game_system(), GameSystem::Starfinder)
        .then(|| localize_loader::lookup_path(localize_loader::sf2e_data(), &dot_path))
        .flatten();

    sf2e_result
        .or_else(|| localize_loader::lookup_path(localize_loader::lang_data(), &dot_path))
        .ok_or_else(|| {
            anyhow!(
                "no data found for condition {} with field {} in both sf2e and pf2e",
                name,
                field
            )
        })
}

#[derive(Debug, Clone)]
pub struct BybeCondition {
    pub name: String,
    pub rule: Description, // take from en.json, ignore description from json
    pub note: Option<Description>,
    pub summary: Option<Description>,
    pub publication_info: PublicationInfo,
    pub overrides: Vec<String>,
    pub is_perpetual: bool,
    pub is_stackable: bool, // value => isvalue = true
    pub value: Option<i64>,
    pub group: Option<String>,
}
