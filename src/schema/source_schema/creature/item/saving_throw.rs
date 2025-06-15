use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SavingThrow {
    pub basic: bool,
    pub statistic: String,
}

#[derive(Debug, Error)]
pub enum SavingThrowParsingError {
    #[error("Basic field is not a valid bool")]
    Basic,
    #[error("Missing statistic field")]
    Statistic,
    #[error("Saving throw json not found")]
    SavingThrow,
}

impl TryFrom<&Value> for SavingThrow {
    type Error = SavingThrowParsingError;

    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        match json.as_object().is_none() {
            false => Ok(SavingThrow {
                basic: json_utils::get_field_from_json(json, "basic")
                    .as_bool()
                    .ok_or(SavingThrowParsingError::Basic)?,
                statistic: json_utils::get_field_from_json(json, "statistic")
                    .as_str()
                    .map(String::from)
                    .ok_or(SavingThrowParsingError::Statistic)?,
            }),
            true => Err(SavingThrowParsingError::SavingThrow),
        }
    }
}
