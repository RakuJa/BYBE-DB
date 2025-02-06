use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SavingThrow {
    pub basic: bool,
    pub statistic: String,
}
impl SavingThrow {
    pub fn init_from_json(json: &Value) -> Option<SavingThrow> {
        match json.as_object().is_none() {
            false => Some(SavingThrow {
                basic: json_utils::get_field_from_json(json, "basic")
                    .as_bool()
                    .unwrap(),
                statistic: json_utils::get_field_from_json(json, "statistic")
                    .as_str()
                    .unwrap()
                    .to_string(),
            }),
            true => None,
        }
    }
}
