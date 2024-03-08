use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawTraits {
    pub rarity: String,
    pub size: String,
    pub traits: Vec<String>,
}

impl RawTraits {
    pub fn init_from_json(json: Value) -> RawTraits {
        RawTraits {
            rarity: json_utils::get_field_from_json(&json, "rarity")
                .as_str()
                .unwrap()
                .to_string(),
            size: json_utils::get_field_from_json(
                &json_utils::get_field_from_json(&json, "size"),
                "value",
            )
            .as_str()
            .unwrap()
            .to_string(),
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(&json, "value")
                    .as_array()
                    .unwrap(),
            ),
        }
    }
}
