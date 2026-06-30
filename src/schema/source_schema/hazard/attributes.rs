use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::hp_values::{HpParsingError, RawHpValues};
use crate::schema::source_schema::common::resistance::{Resistance, ResistanceParserError};
use crate::utils::json_utils;
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use tracing::warn;

#[derive(Debug)]
pub struct RawAttributes {
    // attributes
    pub ac: Option<i64>, //i8,
    pub ac_details: Option<String>,
    pub speed: HashMap<String, i64>,
    pub immunities: Vec<String>,
    pub resistances: Vec<Resistance>,
    pub weakness: HashMap<String, i64>,
    pub stealth: i64,
    pub stealth_detail: Description,
    pub hardness: i64,
    pub has_health: bool,
    pub hp_values: Option<RawHpValues>,
}

#[derive(Debug, Error)]
pub enum AttributeParsingError {
    #[error("Mandatory AC detail is missing from json")]
    ResistanceError(#[from] ResistanceParserError),
    #[error("Hp fields (value or details) could not be parsed")]
    HpError(#[from] HpParsingError),
}

impl TryFrom<&Value> for RawAttributes {
    type Error = AttributeParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let ac_json = get_field_from_json(json, "ac");
        let hp_json = get_field_from_json(json, "hp");

        let speed_json = get_field_from_json(json, "speed");
        let tmp_speed_map = json_utils::from_json_vec_of_maps_to_map(&speed_json, "otherSpeeds");
        let mut speed_map = tmp_speed_map.unwrap_or_default();
        if let Some(speed_val) = get_field_from_json(&speed_json, "value").as_i64() {
            speed_map.insert("Base".to_string(), speed_val);
        }

        let stealth_num = get_field_from_json(&get_field_from_json(json, "stealth"), "value").as_i64().unwrap_or(0);

        let hp_values = RawHpValues::try_from(&hp_json).ok();

        let has_health = get_field_from_json(json, "hasHealth")
            .as_bool()
            .unwrap_or(hp_values.as_ref().is_some_and(|x| x.hp > 0));

        Ok(RawAttributes {
            ac: ac_json.get("value").and_then(|x| x.as_i64()),
            ac_details: ac_json
                .get("details")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string()),
            hp_values,
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
            hardness: get_field_from_json(json, "hardness").as_i64().unwrap_or(0),
            has_health,
            stealth: stealth_num,
            stealth_detail: get_field_from_json(&get_field_from_json(json, "stealth"), "details")
                .as_str()
                .map(Description::from)
                .unwrap(),
        })
    }
}
