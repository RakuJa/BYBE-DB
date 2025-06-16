use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::description::Description;
use crate::schema::source_schema::rules::{Rule, RuleParseError};
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub action_type: String,
    pub n_of_actions: Option<i64>,
    pub category: Option<String>,
    pub description: String,
    pub rules: Vec<Rule>,
    pub publication_info: PublicationInfo,
    pub slug: Option<String>,
    pub traits: ActionTraits,
}

#[derive(Debug, Error)]
pub enum ActionParsingError {
    #[error("Missing action name")]
    Name,
    #[error("Missing action type")]
    ActionType,
    #[error("Missing action description")]
    Description,
    #[error("Missing rarity")]
    Rarity,
    #[error("Missing trait")]
    Trait,
    #[error("publication could not be parsed")]
    PublicationError(#[from] PublicationParsingError),
    #[error("Rule could not be parsed")]
    RuleError(#[from] RuleParseError),
}

impl TryFrom<&Value> for Action {
    type Error = ActionParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let system_json = json_utils::get_field_from_json(json, "system");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        let action_type_json = json_utils::get_field_from_json(&system_json, "actionType");
        let action_json = json_utils::get_field_from_json(&system_json, "actions");
        let category_json = json_utils::get_field_from_json(&system_json, "category");
        let description_json = json_utils::get_field_from_json(&system_json, "description");
        let rule_json = json_utils::get_field_from_json(&system_json, "rules");
        let slug_json = json_utils::get_field_from_json(&system_json, "slug");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        Ok(Action {
            name: json
                .get("name")
                .and_then(Value::as_str)
                .map(String::from)
                .ok_or(ActionParsingError::Name)?,
            action_type: json_utils::get_field_from_json(&action_type_json, "value")
                .as_str()
                .map(String::from)
                .ok_or(ActionParsingError::ActionType)?,
            n_of_actions: json_utils::get_field_from_json(&action_json, "value").as_i64(),
            category: category_json.as_str().map(|x| x.to_string()),
            description: json_utils::get_field_from_json(&description_json, "value")
                .as_str()
                .map(Description::from)
                .map(|x| x.to_string())
                .ok_or(ActionParsingError::Description)?,
            rules: rule_json
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(Rule::try_from)
                .filter(|x| x.is_ok())
                .collect::<Result<Vec<Rule>, _>>()?,
            publication_info: PublicationInfo::try_from(&publication_json)?,
            slug: slug_json.as_str().map(|x| x.to_string()),
            traits: ActionTraits::try_from(&traits_json)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ActionTraits {
    pub rarity: String,
    pub traits: Vec<String>,
}

impl TryFrom<&Value> for ActionTraits {
    type Error = ActionParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(ActionTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .map(String::from)
                .ok_or(ActionParsingError::Rarity)?,
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "value")
                    .as_array()
                    .ok_or(ActionParsingError::Trait)?,
            ),
        })
    }
}
