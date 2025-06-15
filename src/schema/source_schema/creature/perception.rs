use crate::schema::source_schema::creature::sense::{Sense, SenseParsingError};
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub struct RawPerception {
    pub perception_modifier: i64,
    pub perception_details: String,
    pub senses: Vec<Sense>,
    pub vision: bool,
}

#[derive(Debug, Error)]
pub enum PerceptionParsingError {
    #[error("Perception modifier is NaN")]
    PerceptionModifierNaN,
    #[error("Perception details could not be parsed")]
    PerceptionDetails,
    #[error("Senses field is not a valid array")]
    Senses,
    #[error("Vision field is not a valid boolean")]
    Vision,
    #[error("Sense could not be parsed")]
    SenseError(#[from] SenseParsingError),
}

impl TryFrom<&Value> for RawPerception {
    type Error = PerceptionParsingError;

    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(RawPerception {
            perception_details: json
                .get("details")
                .and_then(|x| x.as_str())
                .map(String::from)
                .ok_or(PerceptionParsingError::PerceptionDetails)?,
            perception_modifier: json_utils::get_field_from_json(json, "mod")
                .as_i64()
                .ok_or(PerceptionParsingError::PerceptionModifierNaN)?,
            senses: json_utils::get_field_from_json(json, "senses")
                .as_array()
                .ok_or(PerceptionParsingError::Senses)?
                .iter()
                .map(Sense::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            vision: json
                .get("vision")
                .map(|b| b.as_bool().ok_or(PerceptionParsingError::Vision))
                .unwrap_or(Ok(true))?,
        })
    }
}
