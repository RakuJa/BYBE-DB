use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub struct RawSaves {
    pub fortitude: i64,
    pub fortitude_detail: String,
    pub reflex: i64,
    pub reflex_detail: String,
    pub will: i64,
    pub will_detail: String,
}

#[derive(Debug, Error)]
pub enum SaveParsingError {
    #[error("Fortitude save is NaN")]
    Fortitude,
    #[error("Fortitude detail field could not be parsed")]
    FortitudeDetail,
    #[error("Reflex save is NaN")]
    Reflex,
    #[error("Reflex detail field could not be parsed")]
    ReflexDetail,
    #[error("Will save is NaN")]
    Will,
    #[error("Will detail field could not be parsed")]
    WillDetail,
}

impl TryFrom<&Value> for RawSaves {
    type Error = SaveParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let fortitude_json = json_utils::get_field_from_json(json, "fortitude");
        let reflex_json = json_utils::get_field_from_json(json, "reflex");
        let will_json = json_utils::get_field_from_json(json, "will");

        Ok(RawSaves {
            fortitude: json_utils::get_field_from_json(&fortitude_json, "value")
                .as_i64()
                .ok_or(SaveParsingError::Fortitude)?,
            fortitude_detail: fortitude_json
                .get("saveDetail")
                .and_then(|x| x.as_str())
                .map(String::from)
                .ok_or(SaveParsingError::FortitudeDetail)?,
            reflex: json_utils::get_field_from_json(&reflex_json, "value")
                .as_i64()
                .ok_or(SaveParsingError::Reflex)?,
            reflex_detail: reflex_json
                .get("saveDetail")
                .and_then(|x| x.as_str())
                .map(String::from)
                .ok_or(SaveParsingError::ReflexDetail)?,
            will: json_utils::get_field_from_json(&will_json, "value")
                .as_i64()
                .ok_or(SaveParsingError::Will)?,
            will_detail: will_json
                .get("saveDetail")
                .and_then(|x| x.as_str())
                .map(String::from)
                .ok_or(SaveParsingError::WillDetail)?,
        })
    }
}
