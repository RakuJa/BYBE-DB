use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawResource {
    pub n_of_focus_points: i64,
}

impl From<&Value> for RawResource {
    fn from(json: &Value) -> Self {
        RawResource {
            n_of_focus_points: json_utils::get_field_from_json(
                &json_utils::get_field_from_json(json, "focus"),
                "max",
            )
            .as_i64()
            .unwrap_or(0),
        }
    }
}
