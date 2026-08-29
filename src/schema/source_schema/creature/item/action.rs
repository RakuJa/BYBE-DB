use crate::game_system_handler::current_game_system;
use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::localize_loader;
use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::frequency::Frequency;
use crate::schema::source_schema::creature::item::aura::{Aura, extract_auras};
use crate::schema::source_schema::creature::item::fast_healing::FastHealing;
use crate::schema::source_schema::creature::item::granted_strike::{
    GrantedStrike, extract_granted_strikes,
};
use crate::schema::source_schema::creature::item::movement_data::{
    MovementData, extract_movement_changes,
};
use crate::schema::source_schema::creature::item::size_change::extract_size_changes;
use crate::schema::source_schema::rules::{Rule, RuleParseError};
use crate::utils::game_system_enum::GameSystem;
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

/// Resolves a Strike rule element's `label` (usually a localization key) into display text,
/// using the same lookup chain `Description::resolve_localization` uses for descriptions.
fn resolve_strike_label(path: &str) -> Option<String> {
    let gs = current_game_system();
    if matches!(gs, GameSystem::Starfinder)
        && let Some(value) = localize_loader::lookup_path(localize_loader::sf2e_data(), path)
    {
        return Some(value);
    }
    localize_loader::lookup_path(localize_loader::lang_data(), path)
        .or_else(|| localize_loader::lookup_path(localize_loader::action_lang_data(), path))
}

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
    pub frequency: Option<Frequency>,
    pub auras: Vec<Aura>,
    pub fast_healing: Option<FastHealing>,
    pub strikes: Vec<GrantedStrike>,
    pub movement_changes: Vec<MovementData>,
    pub creature_sizes: Vec<SizeEnum>,
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
        let frequency_json = json_utils::get_field_from_json(&system_json, "frequency");
        let raw_description = json_utils::get_field_from_json(&description_json, "value")
            .as_str()
            .map(String::from)
            .ok_or(ActionParsingError::Description)?;

        Ok(Action {
            name: json
                .get("name")
                .and_then(Value::as_str)
                .map(String::from)
                .ok_or(ActionParsingError::Name)
                .unwrap(),
            action_type: json_utils::get_field_from_json(&action_type_json, "value")
                .as_str()
                .map(String::from)
                .ok_or(ActionParsingError::ActionType)
                .unwrap(),
            n_of_actions: json_utils::get_field_from_json(&action_json, "value").as_i64(),
            category: category_json.as_str().map(|x| x.to_string()),
            description: Description::from(raw_description).resolve(None),
            rules: rule_json
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(Rule::try_from)
                .filter(|x| x.is_ok())
                .collect::<Result<Vec<Rule>, _>>()
                .unwrap(),
            auras: extract_auras(rule_json.as_array().unwrap_or(&vec![])),
            fast_healing: FastHealing::init_from_json(rule_json.as_array().unwrap_or(&vec![])),
            strikes: extract_granted_strikes(
                rule_json.as_array().unwrap_or(&vec![]),
                resolve_strike_label,
            ),
            movement_changes: extract_movement_changes(rule_json.as_array().unwrap_or(&vec![])),
            creature_sizes: extract_size_changes(rule_json.as_array().unwrap_or(&vec![])),
            publication_info: PublicationInfo::try_from(&publication_json).unwrap(),
            slug: slug_json.as_str().map(|x| x.to_string()),
            traits: ActionTraits::try_from(&traits_json).unwrap(),
            frequency: Frequency::try_from(&frequency_json).ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ActionTraits {
    pub rarity: RarityEnum,
    pub traits: Vec<String>,
}

impl TryFrom<&Value> for ActionTraits {
    type Error = ActionParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(ActionTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .map(String::from)
                .ok_or(ActionParsingError::Rarity)
                .and_then(|rarity| rarity.parse().map_err(|_| ActionParsingError::Rarity))
                .unwrap_or_default(),
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "value")
                    .as_array()
                    .ok_or(ActionParsingError::Trait)?,
            ),
        })
    }
}
