use crate::utils::json_utils;
use serde_json::Value;

pub struct RawMaterial {
    pub grade: Option<String>,
    pub m_type: Option<String>,
}

impl From<&Value> for RawMaterial {
    fn from(json: &Value) -> Self {
        RawMaterial {
            grade: json_utils::get_field_from_json(json, "grade")
                .as_str()
                .map(|x| x.to_string()),
            m_type: json_utils::get_field_from_json(json, "type")
                .as_str()
                .map(|x| x.to_string()),
        }
    }
}
