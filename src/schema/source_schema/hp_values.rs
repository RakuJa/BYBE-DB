use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawHpValues {
    pub hp: i64,
    pub max_hp: i64,
    pub temp_hp: i64,
}

impl RawHpValues {
    pub fn init_from_json(json: &Value) -> RawHpValues {
        let hp = json.get("max").unwrap_or(json.get("value").unwrap());
        RawHpValues {
            hp: hp
                .as_i64()
                .unwrap_or_else(|| hp.as_str().unwrap().parse::<i64>().expect("HP field NaN")),
            max_hp: json.get("max").unwrap().as_i64().expect("MAX HP field NaN"),
            temp_hp: json_utils::get_field_from_json(json, "temp")
                .as_i64()
                .unwrap_or(0),
        }
    }
}
