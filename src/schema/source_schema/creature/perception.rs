use crate::schema::source_schema::creature::sense::Sense;
use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawPerception {
    pub perception_modifier: i64,
    pub perception_details: String,
    pub senses: Vec<Sense>,
    pub vision: bool,
}

impl RawPerception {
    pub fn init_from_json(json: Value) -> RawPerception {
        RawPerception {
            perception_details: json.get("details").unwrap().as_str().unwrap().to_string(),
            perception_modifier: json_utils::get_field_from_json(&json, "mod")
                .as_i64()
                .expect("Perception mod NaN"),
            senses: json_utils::get_field_from_json(&json, "senses")
                .as_array()
                .unwrap()
                .iter()
                .map(Sense::init_from_json)
                .collect(),
            vision: json
                .get("vision")
                .map(|b| b.as_bool().unwrap())
                .unwrap_or(true),
        }
    }
}
