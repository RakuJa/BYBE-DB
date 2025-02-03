use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Sense {
    pub name: String, //type
    pub range: Option<i64>,
    pub acuity: Option<String>,
}

impl Sense {
    pub fn init_from_json(json: &Value) -> Sense {
        Sense {
            name: json.get("type").unwrap().as_str().unwrap().to_string(),
            range: json.get("range").map(|n| n.as_i64().unwrap()),
            acuity: json_utils::get_field_from_json(json, "acuity")
                .as_str()
                .map(|s| s.to_string()),
        }
    }
}
