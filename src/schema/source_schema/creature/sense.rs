use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Sense {
    pub name: String, //type
    pub range: Option<i64>,
    pub acuity: Option<String>,
}

#[derive(Debug, Error)]
pub enum SenseParsingError {
    #[error("Type field, representing name, is invalid")]
    Name,
    #[error("Range field is NaN")]
    RangeNaN,
}

impl TryFrom<&Value> for Sense {
    type Error = SenseParsingError;

    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(Sense {
            name: json
                .get("type")
                .and_then(|x| x.as_str())
                .map(String::from)
                .ok_or(SenseParsingError::Name)?,
            range: if let Some(range) = json.get("range") {
                Some(range.as_i64().ok_or(SenseParsingError::RangeNaN)?)
            } else {
                None
            },
            acuity: json_utils::get_field_from_json(json, "acuity")
                .as_str()
                .map(|s| s.to_string()),
        })
    }
}
