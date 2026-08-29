use crate::schema::source_schema::common::hp_values::RawHpValues;
use crate::schema::source_schema::common::resistance::{Resistance, ResistanceParserError};
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
    pub speed: HashMap<String, i64>,
    pub speed_details: String,
    pub immunities: Vec<String>,
    pub resistances: Vec<Resistance>,
    pub weakness: HashMap<String, i64>,
}

#[derive(Debug, Error)]
pub enum AttributeParsingError {
    #[error("AC field is missing or NaN")]
    AC,
    #[error("Mandatory AC detail is missing from json")]
    ACDetail,
    #[error("Resistance could not be parsed")]
    ResistanceError(#[from] ResistanceParserError),
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
            hp_values: RawHpValues::from(&hp_json),
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
            speed_details: json_utils::get_field_from_json(&speed_json, "details")
                .as_str()
                .unwrap_or_default()
                .to_string(),
            weakness: json_utils::from_json_vec_of_maps_to_map(json, "weaknesses")
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn speed_details_is_parsed_when_present() {
        let json = json!({
            "ac": { "value": 20, "details": "" },
            "hp": { "value": 10, "max": 10 },
            "speed": { "value": 20, "details": "earth glide" },
        });
        let attributes = RawAttributes::try_from(&json).unwrap();
        assert_eq!("earth glide", attributes.speed_details);
    }

    #[test]
    fn speed_details_defaults_to_empty_when_absent() {
        let json = json!({
            "ac": { "value": 20, "details": "" },
            "hp": { "value": 10, "max": 10 },
            "speed": { "value": 20 },
        });
        let attributes = RawAttributes::try_from(&json).unwrap();
        assert_eq!("", attributes.speed_details);
    }
}
