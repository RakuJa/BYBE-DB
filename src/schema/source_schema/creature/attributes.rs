use crate::utils::json_utils;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawAttributes {
    // attributes
    pub ac: i64, //i8,
    pub ac_details: String,
    pub hp: i64, //i16,
    pub max_hp: i64,
    pub temp_hp: i64,
    pub hp_details: String,
    pub speed: HashMap<String, i64>,
    pub immunities: Vec<String>,
    pub resistances: HashMap<String, i64>,
    pub weakness: HashMap<String, i64>,
}

impl RawAttributes {
    pub fn init_from_json(json: Value) -> RawAttributes {
        let ac_json = json_utils::get_field_from_json(&json, "ac");
        let hp_json = json_utils::get_field_from_json(&json, "hp");

        let speed_json = json_utils::get_field_from_json(&json, "speed");
        let tmp_speed_map = json_utils::from_json_vec_of_maps_to_map(&speed_json, "otherSpeeds");
        let mut speed_map = tmp_speed_map.unwrap_or_default();
        speed_map.insert(
            "Base".to_string(),
            json_utils::get_field_from_json(&speed_json, "value")
                .as_i64()
                .unwrap_or(0),
        );
        let resistances_map = json_utils::from_json_vec_of_maps_to_map(&json, "resistances");
        let weaknesses_map = json_utils::from_json_vec_of_maps_to_map(&json, "weaknesses");
        let tmp_hp = hp_json.get("max").unwrap_or(hp_json.get("value").unwrap());
        RawAttributes {
            ac: ac_json
                .get("value")
                .unwrap()
                .as_i64()
                .expect("AC field NaN"),
            ac_details: ac_json
                .get("details")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            hp: tmp_hp.as_i64().unwrap_or_else(|| {
                tmp_hp
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .expect("HP field NaN")
            }),
            max_hp: hp_json
                .get("max")
                .unwrap()
                .as_i64()
                .expect("MAX HP field NaN"),
            temp_hp: hp_json.get("temp").unwrap().as_i64().unwrap_or(0),
            hp_details: hp_json
                .get("details")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            immunities: json_utils::from_json_vec_of_jsons_convert_to_array_of_str(
                &json,
                "immunities",
                "type",
            ),
            resistances: resistances_map.unwrap_or_default(),
            speed: speed_map,
            weakness: weaknesses_map.unwrap_or_default(),
        }
    }
}
