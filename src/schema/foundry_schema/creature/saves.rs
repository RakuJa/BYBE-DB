use crate::schema::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawSaves {
    pub fortitude: i64,
    pub fortitude_detail: String,
    pub reflex: i64,
    pub reflex_detail: String,
    pub will: i64,
    pub will_detail: String,
}

impl RawSaves {
    pub fn init_from_json(json: Value) -> RawSaves {
        let fortitude_json = json_utils::get_field_from_json(&json, "fortitude");
        let reflex_json = json_utils::get_field_from_json(&json, "reflex");
        let will_json = json_utils::get_field_from_json(&json, "will");

        RawSaves {
            fortitude: json_utils::get_field_from_json(&fortitude_json, "value")
                .as_i64()
                .expect("Fortitude save is NaN"),
            fortitude_detail: fortitude_json
                .get("saveDetail")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            reflex: json_utils::get_field_from_json(&reflex_json, "value")
                .as_i64()
                .expect("Reflex save is NaN"),
            reflex_detail: reflex_json
                .get("saveDetail")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            will: json_utils::get_field_from_json(&will_json, "value")
                .as_i64()
                .expect("Will save is NaN"),
            will_detail: will_json
                .get("saveDetail")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
