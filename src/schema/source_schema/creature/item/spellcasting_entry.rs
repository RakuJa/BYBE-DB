use crate::schema::source_schema::creature::item::spell::Spell;
use crate::utils::json_utils;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct SpellCastingEntry {
    pub name: String,

    pub is_flexible: Option<bool>,
    pub type_of_spellcaster: String,

    pub dc_modifier: i64,
    pub atk_modifier: i64,
    pub tradition: String,
    pub spell_slots: HashMap<i64, Vec<Spell>>,

    pub heighten_level: i64,
}

impl From<(&RawSpellCastingEntry, &Vec<Spell>, i64)> for SpellCastingEntry {
    fn from(tuple: (&RawSpellCastingEntry, &Vec<Spell>, i64)) -> Self {
        let raw = tuple.0;
        let spells = tuple.1;
        let cr_lvl = tuple.2;

        let mut spell_slots: HashMap<_, _> = raw
            .raw_spell_slots
            .iter()
            .filter_map(|(k, v)| {
                let spells_for_slot: Vec<_> = spells
                    .iter()
                    .filter(|s| v.contains(&s.raw_foundry_id))
                    .cloned()
                    .collect();
                (!spells_for_slot.is_empty()).then_some((*k, spells_for_slot))
            })
            .collect();

        let used_spells: Vec<_> = spell_slots.values().flatten().collect();
        let unused_spells: Vec<_> = spells
            .iter()
            .filter(|spell| !used_spells.contains(spell))
            .collect();
        for spell in unused_spells {
            let slot = if spell.traits.traits.contains(&String::from("cantrip")) {
                0
            } else if let Some(x) = spell.heightened_level {
                x
            } else {
                spell.level
            };

            spell_slots
                .entry(slot)
                .and_modify(|v| v.push(spell.clone()))
                .or_insert(vec![spell.clone()]);
        }
        let level = raw
            .heighten_level
            .unwrap_or_else(|| 10.min((cr_lvl as f64 / 2.).ceil() as i64));
        Self {
            name: raw.name.to_string(),
            is_flexible: raw.is_flexible,
            type_of_spellcaster: raw.type_of_spellcaster.to_string(),
            dc_modifier: raw.dc_modifier,
            atk_modifier: raw.atk_modifier,
            tradition: raw.tradition.to_string(),
            heighten_level: level,
            spell_slots,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawSpellCastingEntry {
    pub raw_foundry_id: String,

    pub name: String,

    pub is_flexible: Option<bool>,
    pub type_of_spellcaster: String,

    pub dc_modifier: i64,
    pub atk_modifier: i64,
    pub tradition: String,
    pub raw_spell_slots: HashMap<i64, Vec<String>>,

    pub heighten_level: Option<i64>,
}

#[derive(Debug, Error)]
pub enum RawSpellCastingParsingError {
    #[error("Mandatory Foundry ID is missing or NaN")]
    FoundryID,
    #[error("Mandatory Name field is missing from json")]
    Name,
    #[error("Mandatory spell caster type field is missing from json")]
    Type,
    #[error("Spell caster dc modifier field could not be parsed")]
    DcModifier,
    #[error("Spell caster atk modifier field could not be parsed")]
    AtkModifier,
    #[error("Tradition field could not be parsed")]
    Tradition,
    #[error("Spell slot field could not be parsed")]
    SpellSlot,
}

impl TryFrom<&Value> for RawSpellCastingEntry {
    type Error = RawSpellCastingParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let system_json = json_utils::get_field_from_json(json, "system");
        let prepared_json = json_utils::get_field_from_json(&system_json, "prepared");
        let spell_dc = json_utils::get_field_from_json(&system_json, "spelldc");
        let heighten_level = json_utils::get_field_from_json(&system_json, "autoHeightenLevel");

        Ok(RawSpellCastingEntry {
            raw_foundry_id: json_utils::get_field_from_json(json, "_id")
                .as_str()
                .map(String::from)
                .ok_or(RawSpellCastingParsingError::FoundryID)?,
            name: json
                .get("name")
                .and_then(Value::as_str)
                .map(String::from)
                .ok_or(RawSpellCastingParsingError::Name)?,
            is_flexible: json_utils::get_field_from_json(&prepared_json, "flexible").as_bool(),
            heighten_level: json_utils::get_field_from_json(&heighten_level, "value").as_i64(),
            type_of_spellcaster: json_utils::get_field_from_json(&prepared_json, "value")
                .as_str()
                .map(String::from)
                .ok_or(RawSpellCastingParsingError::Type)?,
            dc_modifier: json_utils::get_field_from_json(&spell_dc, "dc")
                .as_i64()
                .ok_or(RawSpellCastingParsingError::DcModifier)?,
            atk_modifier: json_utils::get_field_from_json(&spell_dc, "value")
                .as_i64()
                .ok_or(RawSpellCastingParsingError::AtkModifier)?,
            tradition: json_utils::get_field_from_json(&system_json, "type")
                .as_str()
                .map(String::from)
                .ok_or(RawSpellCastingParsingError::Tradition)?,
            raw_spell_slots: json_utils::get_field_from_json(&system_json, "slots")
                .as_object()
                .ok_or(RawSpellCastingParsingError::SpellSlot)?
                .iter()
                .map(|(key, array_of_spells)| {
                    let (_, v) = key.split_at(4);
                    (
                        v.parse::<i64>().unwrap(),
                        json_utils::extract_vec_of_str_from_json_with_vec_of_jsons(
                            array_of_spells,
                            "prepared",
                            "id",
                        ),
                    )
                })
                .collect(),
        })
    }
}
