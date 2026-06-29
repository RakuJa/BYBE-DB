use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct RawHpValues {
    pub hp: i64,
    pub hp_details: Option<String>, // items have no details
}

impl Default for RawHpValues {
    fn default() -> Self {
        Self {
            hp: -2,
            hp_details: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum HpParsingError {
    #[error("Level field missing")]
    HpNaN,
}

impl TryFrom<&Value> for RawHpValues {
    type Error = HpParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let fallback_hp = get_field_from_json(json, "value");
        let hp = json.get("max").unwrap_or(&fallback_hp);
        let hp_details = json
            .get("details")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        if let Some(hp_val) = hp.as_i64() {
            Ok(RawHpValues {
                hp: hp_val,
                hp_details,
            })
        } else {
            Ok(RawHpValues {
                hp: hp
                    .as_str()
                    .ok_or(HpParsingError::HpNaN)?
                    .parse::<i64>()
                    .unwrap_or(0),
                hp_details,
            })
        }
    }
}
