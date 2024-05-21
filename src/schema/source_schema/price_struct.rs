use crate::utils::json_utils;
use serde_json::Value;

pub struct PriceStruct {
    pp: i64,
    gp: i64,
    sp: i64,
    cp: i64,
}

impl PriceStruct {
    pub fn init_from_json(json: &Value) -> PriceStruct {
        let prices = json_utils::get_field_from_json(json, "value");
        PriceStruct {
            pp: json_utils::get_field_from_json(&prices, "pp")
                .as_i64()
                .unwrap_or(0),
            gp: json_utils::get_field_from_json(&prices, "gp")
                .as_i64()
                .unwrap_or(0),
            sp: json_utils::get_field_from_json(&prices, "sp")
                .as_i64()
                .unwrap_or(0),
            cp: json_utils::get_field_from_json(&prices, "cp")
                .as_i64()
                .unwrap_or(0),
        }
    }

    pub fn to_cp(&self) -> i64 {
        self.pp * 1000 + self.gp * 100 + self.sp * 10 + self.cp
    }
}
