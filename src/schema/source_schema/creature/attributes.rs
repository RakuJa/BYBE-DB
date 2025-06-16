use crate::schema::source_schema::creature::resistance::{Resistance, ResistanceParserError};
use crate::schema::source_schema::hp_values::{HpParsingError, RawHpValues};
use crate::utils::json_utils;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug)]
pub struct RawAttributes {
    // attributes
    pub ac: i64, //i8,
    pub ac_details: String,
    pub hp_values: RawHpValues,
    pub hp_details: String,
    pub speed: HashMap<String, i64>,
    pub immunities: Vec<String>,
    pub resistances: Vec<Resistance>,
    pub weakness: HashMap<String, i64>,
}

#[derive(Debug, Error)]
pub enum AttributeParsingError {
    #[error("AC field is missing or NaN")]
    AC,
    #[error("Mandatory Name field is missing from json")]
    HPDetail,
    #[error("Mandatory Name field is missing from json")]
    ACDetail,
    #[error("Source item could not be parsed")]
    ResistanceError(#[from] ResistanceParserError),
    #[error("Hp Value could not be parsed")]
    HpError(#[from] HpParsingError),
}

impl TryFrom<&Value> for RawAttributes {
    type Error = AttributeParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let ac_json = json_utils::get_field_from_json(json, "ac");
        let hp_json = json_utils::get_field_from_json(json, "hp");

        let speed_json = json_utils::get_field_from_json(json, "speed");
        let tmp_speed_map = json_utils::from_json_vec_of_maps_to_map(&speed_json, "otherSpeeds");
        let mut speed_map = tmp_speed_map.unwrap_or_default();
        speed_map.insert(
            "Base".to_string(),
            json_utils::get_field_from_json(&speed_json, "value")
                .as_i64()
                .unwrap_or(0),
        );
        Ok(RawAttributes {
            ac: ac_json
                .get("value")
                .and_then(|x| x.as_i64())
                .ok_or(AttributeParsingError::AC)?,
            ac_details: ac_json
                .get("details")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string())
                .ok_or(AttributeParsingError::ACDetail)?,
            hp_values: RawHpValues::try_from(&hp_json)?,
            hp_details: hp_json
                .get("details")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string())
                .ok_or(AttributeParsingError::HPDetail)?,
            immunities: json_utils::extract_vec_of_str_from_json_with_vec_of_jsons(
                json,
                "immunities",
                "type",
            ),
            resistances: json
                .get("resistances")
                .and_then(|r| r.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(Resistance::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            speed: speed_map,
            weakness: json_utils::from_json_vec_of_maps_to_map(json, "weaknesses")
                .unwrap_or_default(),
        })
    }
}
