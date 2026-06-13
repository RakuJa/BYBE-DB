use crate::schema::source_schema::common::range_data::RangeData;
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Sense {
    pub name: String, //type
    pub range: Option<RangeData>,
    pub acuity: Option<String>,
}

#[derive(Debug, Error)]
pub enum SenseParsingError {
    #[error("Type field, representing name, is invalid")]
    Name,
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
            range: RangeData::try_from(&get_field_from_json(json, "range")).ok(),
            acuity: get_field_from_json(json, "acuity")
                .as_str()
                .map(|s| s.to_string()),
        })
    }
}
