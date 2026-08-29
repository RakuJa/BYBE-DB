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
        let tradition_json = json_utils::get_field_from_json(&system_json, "tradition");

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
            tradition: json_utils::get_field_from_json(&tradition_json, "value")
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tradition_is_read_from_tradition_value_not_type() {
        let json = json!({
            "_id": "abc123",
            "name": "Innate Spells",
            "system": {
                "prepared": { "value": "innate" },
                "spelldc": { "dc": 40, "value": 30 },
                "autoHeightenLevel": { "value": null },
                "tradition": { "value": "arcane" },
                "slots": {},
            }
        });
        let entry = RawSpellCastingEntry::try_from(&json).unwrap();
        assert_eq!("arcane", entry.tradition);
    }

    #[test]
    fn tradition_reads_divine_not_prepared_value() {
        let json = json!({
            "_id": "abc123",
            "name": "Divine Spells",
            "system": {
                "prepared": { "value": "prepared" },
                "spelldc": { "dc": 30, "value": 20 },
                "autoHeightenLevel": { "value": null },
                "tradition": { "value": "divine" },
                "slots": {},
            }
        });
        let entry = RawSpellCastingEntry::try_from(&json).unwrap();
        assert_eq!("divine", entry.tradition);
        assert_eq!("prepared", entry.type_of_spellcaster);
    }

    // Demilich's "Arcane Innate Spells" entry
    #[test]
    fn tradition_parses_from_real_demilich_json() {
        let json = json!({
            "_id": "rNMUpQjQil41A67R",
            "name": "Arcane Innate Spells",
            "system": {
                "autoHeightenLevel": { "value": null },
                "description": { "value": "" },
                "prepared": { "flexible": false, "value": "innate" },
                "proficiency": { "value": 0 },
                "publication": { "license": "OGL", "remaster": false, "title": "" },
                "rules": [],
                "showSlotlessLevels": { "value": false },
                "slots": {},
                "slug": null,
                "spelldc": { "dc": 40, "mod": 0, "value": 30 },
                "tradition": { "value": "arcane" }
            },
            "type": "spellcastingEntry"
        });
        let entry = RawSpellCastingEntry::try_from(&json).unwrap();
        assert_eq!("arcane", entry.tradition);
        assert_eq!("innate", entry.type_of_spellcaster);
        assert_eq!(40, entry.dc_modifier);
        assert_eq!(30, entry.atk_modifier);
    }
}
