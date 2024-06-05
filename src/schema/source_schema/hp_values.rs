use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;

#[derive(Debug)]
pub struct RawHpValues {
    pub hp: i64,
}

impl RawHpValues {
    pub fn init_from_json(json: &Value) -> RawHpValues {
        let fallback_hp = get_field_from_json(json, "value");
        let hp = json.get("max").unwrap_or(&fallback_hp);
        RawHpValues {
            hp: hp
                .as_i64()
                .unwrap_or_else(|| hp.as_str().unwrap().parse::<i64>().unwrap_or(0)),
        }
    }
}
