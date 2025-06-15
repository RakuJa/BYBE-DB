use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub struct RawAbilities {
    pub charisma: i64,
    pub constitution: i64,
    pub dexterity: i64,
    pub intelligence: i64,
    pub strength: i64,
    pub wisdom: i64,
}

#[derive(Debug, Error)]
pub enum AbilityParsingError {
    #[error("Charisma modifier is NaN")]
    Charisma,
    #[error("Constitution  modifier is NaN")]
    Constitution,
    #[error("Dexterity modifier is NaN")]
    Dexterity,
    #[error("Intelligence  modifier is NaN")]
    Intelligence,
    #[error("Strength modifier is NaN")]
    Strength,
    #[error("Wisdom modifier is NaN")]
    Wisdom,
}

impl TryFrom<&Value> for RawAbilities {
    type Error = AbilityParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(RawAbilities {
            charisma: get_ability_modifier(json_utils::get_field_from_json(json, "cha"))
                .ok_or(AbilityParsingError::Charisma)?,
            constitution: get_ability_modifier(json_utils::get_field_from_json(json, "con"))
                .ok_or(AbilityParsingError::Constitution)?,
            dexterity: get_ability_modifier(json_utils::get_field_from_json(json, "dex"))
                .ok_or(AbilityParsingError::Dexterity)?,
            intelligence: get_ability_modifier(json_utils::get_field_from_json(json, "int"))
                .ok_or(AbilityParsingError::Intelligence)?,
            strength: get_ability_modifier(json_utils::get_field_from_json(json, "str"))
                .ok_or(AbilityParsingError::Strength)?,
            wisdom: get_ability_modifier(json_utils::get_field_from_json(json, "wis"))
                .ok_or(AbilityParsingError::Wisdom)?,
        })
    }
}

fn get_ability_modifier(modifier: Value) -> Option<i64> {
    let value = modifier.get("mod")?;
    if value.is_null() {
        // For some reason null is used to represent 0
        Some(0)
    } else {
        value.as_i64()
    }
}
