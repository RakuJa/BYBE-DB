use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawAbilities {
    pub charisma: i64,
    pub constitution: i64,
    pub dexterity: i64,
    pub intelligence: i64,
    pub strength: i64,
    pub wisdom: i64,
}

impl RawAbilities {
    pub fn init_from_json(json: Value) -> RawAbilities {
        RawAbilities {
            charisma: get_ability_modifier(json_utils::get_field_from_json(&json, "cha"))
                .expect("Cha Modifier is NaN"),
            constitution: get_ability_modifier(json_utils::get_field_from_json(&json, "con"))
                .expect("Con Modifier is NaN"),
            dexterity: get_ability_modifier(json_utils::get_field_from_json(&json, "dex"))
                .expect("Dex Modifier is NaN"),
            intelligence: get_ability_modifier(json_utils::get_field_from_json(&json, "int"))
                .expect("Int Modifier is NaN"),
            strength: get_ability_modifier(json_utils::get_field_from_json(&json, "str"))
                .expect("Str Modifier is NaN"),
            wisdom: get_ability_modifier(json_utils::get_field_from_json(&json, "wis"))
                .expect("Wis Modifier is NaN"),
        }
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
