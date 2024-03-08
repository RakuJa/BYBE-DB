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
            charisma: json_utils::get_field_from_json(&json, "cha")
                .get("mod")
                .unwrap()
                .as_i64()
                .expect("Cha Modifier is NaN"),
            constitution: json_utils::get_field_from_json(&json, "con")
                .get("mod")
                .unwrap()
                .as_i64()
                .expect("Con Modifier is NaN"),
            dexterity: json_utils::get_field_from_json(&json, "dex")
                .get("mod")
                .unwrap()
                .as_i64()
                .expect("Dex Modifier is NaN"),
            intelligence: json_utils::get_field_from_json(&json, "int")
                .get("mod")
                .unwrap()
                .as_i64()
                .expect("Int Modifier is NaN"),
            strength: json_utils::get_field_from_json(&json, "str")
                .get("mod")
                .unwrap()
                .as_i64()
                .expect("Str Modifier is NaN"),
            wisdom: json_utils::get_field_from_json(&json, "wis")
                .get("mod")
                .unwrap()
                .as_i64()
                .expect("Wis Modifier is NaN"),
        }
    }
}
