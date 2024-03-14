use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SpellCastingEntry {
    pub name: String,

    pub is_flexible: Option<bool>,
    pub type_of_spell_caster: String,

    pub dc_modifier: Option<i64>,
    pub atk_modifier: Option<i64>,
    pub modifier: Option<i64>,
    pub item_mod: Option<i64>,
    pub tradition: String,
}

impl SpellCastingEntry {
    pub fn init_from_json(json: Value) -> SpellCastingEntry {
        let system_json = json_utils::get_field_from_json(&json, "system");
        let prepared_json = json_utils::get_field_from_json(&system_json, "prepared");
        let spell_dc = json_utils::get_field_from_json(&system_json, "spelldc");

        SpellCastingEntry {
            name: json_utils::get_field_from_json(&json, "name")
                .as_str()
                .unwrap()
                .to_string(),
            is_flexible: json_utils::get_field_from_json(&prepared_json, "flexible").as_bool(),
            type_of_spell_caster: json_utils::get_field_from_json(&prepared_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            dc_modifier: json_utils::get_field_from_json(&spell_dc, "dc").as_i64(),
            modifier: json_utils::get_field_from_json(&spell_dc, "mod").as_i64(),
            atk_modifier: json_utils::get_field_from_json(&spell_dc, "value").as_i64(),
            item_mod: json_utils::get_field_from_json(&spell_dc, "item").as_i64(),
            tradition: json_utils::get_field_from_json(&system_json, "type")
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
