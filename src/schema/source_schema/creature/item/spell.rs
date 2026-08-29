use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::range_data::RangeData;
use crate::schema::source_schema::common::resistance::ResistanceParserError;
use crate::schema::source_schema::creature::item::saving_throw::SavingThrow;
use crate::utils::dice::formula::{ParsedDiceFormula, parse_dice_formula};
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub raw_foundry_id: String,
    pub location_id: String,
    pub name: String,
    pub description: Description,
    pub area: Option<Area>,
    pub counteraction: bool,

    pub damage: Vec<RawDamageData>,

    pub saving_throw: Option<SavingThrow>,

    pub sustained: bool,
    pub duration: Option<String>,

    pub heightening: Option<Heightening>,
    pub level: i64,
    pub heightened_level: Option<i64>,
    pub range: Option<RangeData>,
    pub target: String,
    pub actions: String,

    pub publication_info: PublicationInfo,
    pub traits: SpellTraits,
}

#[derive(Debug, Error)]
pub enum SpellParsingError {
    #[error("Foundry ID cannot be parsed")]
    FoundryId,
    #[error("Spell slot location cannot be parsed")]
    SlotId,
    #[error("Mandatory Name field is missing from json")]
    Name,
    #[error("Mandatory sustained field is missing from json")]
    Sustained,
    #[error("Mandatory level field is missing from json")]
    Level,
    #[error("Mandatory target field is missing from json")]
    Target,
    #[error("Mandatory action field is missing from json")]
    Action,
    #[error("Area field is missing from json")]
    AreaMissingField,
    #[error("Area type field is missing from json")]
    AreaType,
    #[error("Area value is missing from json")]
    AreaValue,
    #[error("Rarity field is missing from json")]
    Rarity,
    #[error("Damage type field is missing from json")]
    DamageType,
    #[error("Damage formula field is missing from json")]
    DamageFormula,
    #[error("Resistance could not be parsed")]
    Resistance(#[from] ResistanceParserError),
    #[error("Publication info could not be parsed")]
    Publication(#[from] PublicationParsingError),
}

impl TryFrom<&Value> for Spell {
    type Error = SpellParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let system_json = json_utils::get_field_from_json(json, "system");
        let mut damage_data = Vec::new();
        let mut i = 0;
        let damage_json = json_utils::get_field_from_json(&system_json, "damage");
        while let Some(dmg_data_json) = damage_json.get(i.to_string().as_str()) {
            damage_data.push(RawDamageData::try_from(dmg_data_json)?);
            i += 1;
        }

        let defense_json = json_utils::get_field_from_json(&system_json, "defense");
        let description_json = json_utils::get_field_from_json(&system_json, "description");
        let duration_json = json_utils::get_field_from_json(&system_json, "duration");
        let level_json = json_utils::get_field_from_json(&system_json, "level");
        let location_json = json_utils::get_field_from_json(&system_json, "location");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        let range_json = json_utils::get_field_from_json(&system_json, "range");
        let target_json = json_utils::get_field_from_json(&system_json, "target");
        let time_json = json_utils::get_field_from_json(&system_json, "time");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        Ok(Spell {
            raw_foundry_id: json_utils::get_field_from_json(json, "_id")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::FoundryId)?,
            location_id: location_json
                .get("value")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or(SpellParsingError::SlotId)?,
            heightened_level: location_json
                .get("heightenedLevel")
                .and_then(|v| v.as_i64()),
            name: json
                .get("name")
                .and_then(Value::as_str)
                .map(String::from)
                .ok_or(SpellParsingError::Name)?,
            description: json_utils::get_field_from_json(&description_json, "value")
                .as_str()
                .map(Description::from)
                .unwrap_or_default(),
            area: match system_json.get("area") {
                Some(x) => Area::try_from(x).ok(),
                None => None,
            },
            counteraction: json_utils::get_field_from_json(&system_json, "counteraction")
                .as_bool()
                .unwrap_or(false),
            damage: damage_data,
            saving_throw: SavingThrow::try_from(&json_utils::get_field_from_json(
                &defense_json,
                "save",
            ))
            .ok(),
            sustained: json_utils::get_field_from_json(&duration_json, "sustained")
                .as_bool()
                .ok_or(SpellParsingError::Sustained)?,
            duration: json_utils::get_field_from_json(&duration_json, "value")
                .as_str()
                .map(String::from),
            heightening: system_json
                .get("heightening")
                .and_then(Heightening::init_from_json),
            level: level_json
                .get("value")
                .and_then(|x| x.as_i64())
                .ok_or(SpellParsingError::Level)?,
            publication_info: PublicationInfo::try_from(&publication_json)?,
            range: RangeData::try_from(&range_json).ok(),
            target: json_utils::get_field_from_json(&target_json, "value")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::Target)?,
            actions: json_utils::get_field_from_json(&time_json, "value")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::Action)?,
            traits: SpellTraits::try_from(&traits_json)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellTraits {
    pub rarity: String,
    pub traditions: Vec<String>,
    pub traits: Vec<String>,
}

impl TryFrom<&Value> for SpellTraits {
    type Error = SpellParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(SpellTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::Rarity)?,
            traditions: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "traditions")
                    .as_array()
                    .unwrap_or(&Vec::new()),
            ),
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "value")
                    .as_array()
                    .unwrap_or(&Vec::new()),
            ),
        })
    }
}

/// Two mutually exclusive ways a spell can scale with a higher casting rank:
/// `"interval"` (every N ranks above the base, add some damage dice e.g. "Heightened (+2)
/// 1d6 additional damage") and `"fixed"` (overrides to target/range/area/damage).
#[derive(Debug, Clone, PartialEq)]
pub enum Heightening {
    Interval {
        interval: i64,
        damage: Vec<ParsedDiceFormula>,
    },
    Fixed {
        levels: Vec<FixedHeighteningLevel>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedHeighteningLevel {
    pub level: i64,
    pub target: Option<String>,
    pub range: Option<String>,
    pub area: Option<Area>,
    pub damage: Vec<RawDamageData>,
}

impl Heightening {
    fn init_from_json(json: &Value) -> Option<Heightening> {
        match json.get("type").and_then(Value::as_str)? {
            "interval" => {
                let interval = json.get("interval").and_then(Value::as_i64)?;
                let damage = json
                    .get("damage")
                    .and_then(Value::as_object)
                    .map(|dice_by_key| {
                        dice_by_key
                            .values()
                            .filter_map(Value::as_str)
                            .filter_map(parse_dice_formula)
                            .collect()
                    })
                    .unwrap_or_default();
                Some(Heightening::Interval { interval, damage })
            }
            "fixed" => {
                let levels: Vec<FixedHeighteningLevel> = json
                    .get("levels")
                    .and_then(Value::as_object)?
                    .iter()
                    .filter_map(|(level_str, overrides)| {
                        let level = level_str.parse::<i64>().ok()?;
                        Some(FixedHeighteningLevel::init_from_json_and_level(
                            level, overrides,
                        ))
                    })
                    .collect();
                (!levels.is_empty()).then_some(Heightening::Fixed { levels })
            }
            _ => None,
        }
    }
}

impl FixedHeighteningLevel {
    fn init_from_json_and_level(level: i64, json: &Value) -> Self {
        Self {
            level,
            target: json
                .get("target")
                .and_then(|t| t.get("value"))
                .and_then(Value::as_str)
                .map(String::from),
            range: json
                .get("range")
                .and_then(|r| r.get("value"))
                .and_then(Value::as_str)
                .map(String::from),
            area: json
                .get("area")
                .and_then(Area::init_heightened_data_from_json),
            damage: json
                .get("damage")
                .and_then(Value::as_object)
                .map(|dmg_by_key| {
                    dmg_by_key
                        .values()
                        .filter_map(|v| RawDamageData::try_from(v).ok())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Area {
    pub area_type: String,
    pub area_value: i64,
}

impl Area {
    /// A fixed-level area override (heightening) doesn't consistently match the base spell data
    /// shape (`Area::try_from`), so it's parsed separately. Real data is a mix: the type key is
    /// `"areaType"` only, `"type"` only, or both (95 / 69 / 92 respectively across the corpus), and
    /// `value` is a JSON number for some entries and a numeric string for others.
    fn init_heightened_data_from_json(json: &Value) -> Option<Self> {
        let area_type = json
            .get("areaType")
            .or_else(|| json.get("type"))
            .and_then(Value::as_str)?
            .to_string();
        let area_value = json.get("value").and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        })?;
        Some(Self {
            area_type,
            area_value,
        })
    }
}

impl TryFrom<&Value> for Area {
    type Error = SpellParsingError;

    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        match json.as_object().is_none() {
            false => Ok(Area {
                area_type: json_utils::get_field_from_json(json, "type")
                    .as_str()
                    .map(String::from)
                    .ok_or(SpellParsingError::AreaType)?,
                area_value: json_utils::get_field_from_json(json, "value")
                    .as_i64()
                    .ok_or(SpellParsingError::AreaValue)?,
            }),
            true => Err(SpellParsingError::AreaMissingField),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct RawDamageData {
    pub category: Option<String>,
    pub n_of_dice: Option<i64>,
    pub die_size: Option<i64>,
    pub bonus_dmg: i64,
    pub kinds: Vec<String>,
    pub dmg_type: String,
}

impl TryFrom<&Value> for RawDamageData {
    type Error = SpellParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let formula = json_utils::get_field_from_json(json, "formula")
            .as_str()
            .map(String::from)
            .ok_or(SpellParsingError::DamageFormula)?;
        let parsed = parse_dice_formula(&formula).unwrap_or_default();

        Ok(RawDamageData {
            category: json_utils::get_field_from_json(json, "category")
                .as_str()
                .map(|x| x.to_string()),
            n_of_dice: parsed.n_of_dice,
            die_size: parsed.die_size,
            bonus_dmg: parsed.bonus_dmg,
            kinds: json_utils::get_field_from_json(json, "kinds")
                .as_array()
                .map(|arr| json_utils::from_json_vec_of_str_to_vec_of_str(arr))
                .unwrap_or_default(),
            dmg_type: json_utils::get_field_from_json(json, "type")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::DamageType)?,
        })
    }
}

#[cfg(test)]
mod heightening_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_a_simple_interval_heightening() {
        // Harm shape: 1d8 additional damage every rank above base
        let json = json!({"damage": {"0": "1d8"}, "interval": 1, "type": "interval"});
        assert_eq!(
            Some(Heightening::Interval {
                interval: 1,
                damage: vec![ParsedDiceFormula {
                    n_of_dice: Some(1),
                    die_size: Some(8),
                    bonus_dmg: 0,
                    unresolved_terms: vec![],
                }],
            }),
            Heightening::init_from_json(&json)
        );
    }

    #[test]
    fn extracts_interval_heightening_with_multiple_damage_instances() {
        // Eclipse Burst shape: two separate damage riders scale together every rank
        let json = json!({
            "damage": {"0": "1d10", "v1pnjhfu6e9akqe0": "1d4"},
            "interval": 1,
            "type": "interval"
        });
        let Some(Heightening::Interval { interval, damage }) = Heightening::init_from_json(&json)
        else {
            panic!("expected Interval heightening");
        };
        assert_eq!(1, interval);
        assert_eq!(2, damage.len());
    }

    #[test]
    fn extracts_a_fixed_target_override() {
        // Confusion shape: heightened to rank 8, it targets more creatures
        let json = json!({
            "levels": { "8": { "target": { "value": "10 creatures" } } },
            "type": "fixed"
        });
        assert_eq!(
            Some(Heightening::Fixed {
                levels: vec![FixedHeighteningLevel {
                    level: 8,
                    target: Some("10 creatures".to_string()),
                    range: None,
                    area: None,
                    damage: vec![],
                }],
            }),
            Heightening::init_from_json(&json)
        );
    }

    #[test]
    fn extracts_a_fixed_damage_override_reusing_the_base_damage_parser() {
        // Acid Splash shape: rank 3 bumps both the direct hit and splash damage
        let json = json!({
            "levels": {
                "3": {
                    "damage": {
                        "0": {"formula": "2d6", "type": "acid", "category": null},
                        "gcovwqxwitqchoin": {"formula": "1", "type": "acid", "category": "splash"}
                    }
                }
            },
            "type": "fixed"
        });
        let Some(Heightening::Fixed { levels }) = Heightening::init_from_json(&json) else {
            panic!("expected Fixed heightening");
        };
        assert_eq!(1, levels.len());
        assert_eq!(3, levels[0].level);
        assert_eq!(2, levels[0].damage.len());
        // reused RawDamageData parser: kinds defaults to empty since overrides never carry one
        assert!(levels[0].damage.iter().all(|d| d.kinds.is_empty()));
    }

    #[test]
    fn extracts_a_fixed_area_override() {
        // Web shape: heightened to rank 4, the burst grows to 20 feet
        let json = json!({
            "levels": { "4": { "area": { "areaType": "burst", "value": "20" } } },
            "type": "fixed"
        });
        let Some(Heightening::Fixed { levels }) = Heightening::init_from_json(&json) else {
            panic!("expected Fixed heightening");
        };
        assert_eq!(
            Some(Area {
                area_type: "burst".to_string(),
                area_value: 20,
            }),
            levels[0].area
        );
    }

    #[test]
    fn extracts_a_fixed_area_override_with_only_the_legacy_type_key() {
        let json = json!({
            "levels": { "4": { "area": { "type": "burst", "value": "20" } } },
            "type": "fixed"
        });
        let Some(Heightening::Fixed { levels }) = Heightening::init_from_json(&json) else {
            panic!("expected Fixed heightening");
        };
        assert_eq!(
            Some(Area {
                area_type: "burst".to_string(),
                area_value: 20,
            }),
            levels[0].area
        );
    }

    #[test]
    fn ignores_an_empty_heightening_block() {
        // Enervation shape on some statblocks: the object is present but conveys nothing
        let json = json!({"damage": {}});
        assert_eq!(None, Heightening::init_from_json(&json));
    }

    #[test]
    fn ignores_unknown_heightening_types() {
        let json = json!({"type": "something-new", "value": 1});
        assert_eq!(None, Heightening::init_from_json(&json));
    }
}
