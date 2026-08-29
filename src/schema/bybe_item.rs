use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::source_schema::common::range_data::RangeData;
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::item::source_item::{SourceItem, SourceItemParsingError};
use crate::schema::status::Status;
use crate::utils::dice::formula::{ParsedDiceFormula, parse_dice_formula};
use crate::utils::json_utils;
use crate::utils::json_utils::get_field_from_json;
use bon::bon;
use capitalize::Capitalize;
use regex::Regex;
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct BybeItem {
    pub name: String,
    pub foundry_id: String,
    pub bulk: f64,
    pub quantity: i64,
    pub base_item: Option<String>,
    pub category: Option<String>,
    pub description: String,
    pub hardness: i64,
    pub hp: i64,
    pub level: i64,
    pub price: i64, // in cp,
    pub usage: Option<String>,
    pub group: Option<String>,
    pub item_type: String,
    pub material_grade: Option<String>,
    pub material_type: Option<String>,
    pub number_of_uses: Option<i64>, // for consumables, for equip set as null.

    // source details (title, license, remastered)
    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub rarity: RarityEnum,
    pub size: SizeEnum,
    pub traits: Vec<String>,

    pub is_derived: bool,

    pub status: Status,
}

#[derive(Debug, Error)]
pub enum BybeItemParsingError {
    #[error("Unsupported item type")]
    UnsupportedItemType,
    #[error("Invalid item type")]
    InvalidItemType,
    #[error("Missing item type")]
    MissingItemType,
    #[error("Property runes is not an Array")]
    PropertyRunes,
    #[error("Ac bonus is NaN")]
    AcBonus,
    #[error("Check penalty is NaN")]
    CheckPenalty,
    #[error("Dexterity cap is NaN")]
    DexterityCap,
    #[error("Speed penalty is NaN")]
    SpeedPenalty,
    #[error("Damage data is not in a valid format")]
    DamageData,
    #[error("Source item could not be parsed")]
    SourceItemError(#[from] SourceItemParsingError),
}
impl TryFrom<(&Value, bool)> for BybeItem {
    type Error = BybeItemParsingError;
    fn try_from(args: (&Value, bool)) -> Result<Self, Self::Error> {
        let (json, is_derived) = args;
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .map(|x| x.to_ascii_lowercase())
            .ok_or(BybeItemParsingError::MissingItemType)?;
        if !matches!(
            item_type.as_str(),
            "equipment" | "consumable" | "ammo" | "treasure" | "backpack"
        ) {
            return Err(BybeItemParsingError::UnsupportedItemType);
        }
        Ok(Self::from((SourceItem::try_from(json)?, is_derived)))
    }
}

impl From<(SourceItem, bool)> for BybeItem {
    fn from(args: (SourceItem, bool)) -> Self {
        let (source_item, is_derived) = args;
        BybeItem {
            status: Status::from(&source_item),
            name: source_item.name,
            foundry_id: source_item.foundry_id,
            bulk: source_item.bulk,
            quantity: source_item.quantity,
            base_item: source_item.base_item,
            category: source_item.category,
            description: source_item.description.resolve(Some(source_item.level)),
            hardness: source_item.hardness,
            hp: source_item.hp_values.hp,
            level: source_item.level,
            price: source_item.price.to_cp(),
            usage: source_item.usage,
            group: source_item.group,
            item_type: source_item.item_type.to_uppercase(),
            material_grade: source_item.material.grade,
            material_type: source_item.material.m_type,
            number_of_uses: source_item.uses,
            license: source_item.publication_info.license,
            remaster: source_item.publication_info.remastered,
            source: source_item.publication_info.source,
            rarity: source_item
                .traits
                .rarity
                .as_str()
                .parse()
                .unwrap_or(RarityEnum::Common),
            size: source_item
                .traits
                .size
                .as_str()
                .parse()
                .unwrap_or(SizeEnum::Medium),
            traits: source_item.traits.traits,
            is_derived,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BybeArmor {
    pub item_core: BybeItem,
    pub ac_bonus: i64,
    pub check_penalty: i64,
    pub dex_cap: i64,
    pub n_of_potency_runes: i64,
    pub property_runes: Vec<String>,
    pub n_of_resilient_runes: i64,
    pub speed_penalty: i64,
    pub strength_required: Option<i64>,
}

impl TryFrom<(&Value, bool)> for BybeArmor {
    type Error = BybeItemParsingError;
    fn try_from(args: (&Value, bool)) -> Result<Self, Self::Error> {
        let (json, is_derived) = args;
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .map(|x| x.to_ascii_lowercase())
            .ok_or(BybeItemParsingError::MissingItemType)?;
        if !item_type.eq("armor") {
            return Err(BybeItemParsingError::InvalidItemType);
        }
        let system_json = get_field_from_json(json, "system");
        let item_core = BybeItem::from((SourceItem::try_from(json)?, is_derived));
        let rune_json = get_field_from_json(&system_json, "runes");
        Ok(BybeArmor {
            item_core,
            ac_bonus: get_field_from_json(&system_json, "acBonus")
                .as_i64()
                .ok_or(BybeItemParsingError::AcBonus)?,
            check_penalty: get_field_from_json(&system_json, "checkPenalty")
                .as_i64()
                .ok_or(BybeItemParsingError::CheckPenalty)?,
            dex_cap: get_field_from_json(&system_json, "dexCap")
                .as_i64()
                .ok_or(BybeItemParsingError::DexterityCap)?,
            n_of_potency_runes: get_field_from_json(&rune_json, "potency")
                .as_i64()
                .unwrap_or(0),
            property_runes: json_utils::from_json_vec_of_str_to_vec_of_str(
                get_field_from_json(&rune_json, "property")
                    .as_array()
                    .ok_or(BybeItemParsingError::PropertyRunes)?,
            ),
            n_of_resilient_runes: get_field_from_json(&rune_json, "resilient")
                .as_i64()
                .unwrap_or(0),
            speed_penalty: get_field_from_json(&system_json, "speedPenalty")
                .as_i64()
                .ok_or(BybeItemParsingError::SpeedPenalty)?,
            strength_required: get_field_from_json(&system_json, "strength").as_i64(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct BybeWeapon {
    pub item_core: BybeItem,
    pub to_hit_bonus: Option<i64>,
    pub damage_data: Vec<WeaponDamageData>,
    pub splash_dmg: Option<i64>,
    pub n_of_potency_runes: i64,
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    pub range: Option<RangeData>,
    pub reload: Option<String>,
    pub weapon_type: String,
    pub attack_effects: Vec<Action>,
}

#[bon]
impl BybeWeapon {
    #[builder]
    pub fn new(source_weapon: SourceWeapon, creature_actions: &[Action]) -> Self {
        let action_slugs = source_weapon.attack_effects;

        let mut attack_effects = vec![];

        for slug in action_slugs {
            if let Some(i) = creature_actions
                .iter()
                .position(|x| x.slug.as_deref() == Some(slug.as_str()))
                .or_else(|| {
                    let name = slug
                        .split('-')
                        .map(|w| w.capitalize())
                        .collect::<Vec<_>>()
                        .join(" ");
                    creature_actions.iter().position(|x| x.name == name)
                })
                && let Some(a) = creature_actions.get(i)
            {
                attack_effects.push(a.clone());
            }
        }

        Self {
            item_core: source_weapon.item_core,
            to_hit_bonus: source_weapon.to_hit_bonus,
            damage_data: source_weapon.damage_data,
            splash_dmg: source_weapon.splash_dmg,
            n_of_potency_runes: source_weapon.n_of_potency_runes,
            n_of_striking_runes: source_weapon.n_of_striking_runes,
            property_runes: source_weapon.property_runes,
            range: source_weapon.range,
            reload: source_weapon.reload,
            weapon_type: source_weapon.weapon_type,
            attack_effects,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SourceWeapon {
    pub item_core: BybeItem,
    pub to_hit_bonus: Option<i64>,
    pub damage_data: Vec<WeaponDamageData>,
    pub splash_dmg: Option<i64>,
    pub n_of_potency_runes: i64,
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    pub range: Option<RangeData>,
    pub reload: Option<String>,
    pub weapon_type: String,
    pub attack_effects: Vec<String>,
}

impl From<SourceWeapon> for BybeWeapon {
    fn from(value: SourceWeapon) -> Self {
        Self {
            item_core: value.item_core,
            to_hit_bonus: value.to_hit_bonus,
            damage_data: value.damage_data,
            splash_dmg: value.splash_dmg,
            n_of_potency_runes: value.n_of_potency_runes,
            n_of_striking_runes: value.n_of_striking_runes,
            property_runes: value.property_runes,
            range: value.range,
            reload: value.reload,
            weapon_type: value.weapon_type,
            attack_effects: vec![],
        }
    }
}

impl TryFrom<(&Value, bool)> for SourceWeapon {
    type Error = BybeItemParsingError;
    fn try_from(args: (&Value, bool)) -> Result<Self, Self::Error> {
        let (json, is_derived) = args;
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .map(|x| x.to_ascii_lowercase())
            .ok_or(BybeItemParsingError::MissingItemType)?;
        if !(item_type.eq("weapon") | item_type.eq("melee")) {
            return Err(BybeItemParsingError::InvalidItemType);
        }
        let system_json = get_field_from_json(json, "system");
        let runes_data = get_field_from_json(&system_json, "runes");
        let hit_bonus_json = get_field_from_json(&system_json, "bonus");
        let wp_type_json = get_field_from_json(&system_json, "weaponType");

        let tmp_weapon_type = get_field_from_json(&wp_type_json, "value")
            .as_str()
            .unwrap_or("")
            .to_string();
        let range = RangeData::try_from(&get_field_from_json(&system_json, "range")).ok();
        let re = Regex::new(r"\d+").unwrap();
        let range_val = if let Some(r) = &range
            && let Some(n) = re.find(r.value.as_str())
        {
            n.as_str().parse().unwrap()
        } else {
            0
        };

        let weapon_type = if tmp_weapon_type.is_empty() {
            String::from(if item_type.eq("melee") {
                if range_val > 0 || range.as_ref().is_some_and(|x| x.increment.is_some()) {
                    "ranged"
                } else {
                    "melee"
                }
            } else {
                "generic"
            })
        } else {
            tmp_weapon_type
        };

        Ok(Self {
            item_core: BybeItem::from((SourceItem::try_from(json)?, is_derived)),
            splash_dmg: get_field_from_json(&get_field_from_json(json, "splashDamage"), "value")
                .as_i64(),
            n_of_potency_runes: get_field_from_json(&runes_data, "potency")
                .as_i64()
                .unwrap_or(0),
            n_of_striking_runes: get_field_from_json(&runes_data, "striking")
                .as_i64()
                .unwrap_or(0),
            property_runes: match runes_data.get("property") {
                Some(x) => json_utils::from_json_vec_of_str_to_vec_of_str(
                    &x.as_array()
                        .map(|v| v.to_vec())
                        .or_else(|| {
                            x.as_object()
                                .map(|d| d.values().cloned().collect::<Vec<_>>())
                        })
                        .ok_or(BybeItemParsingError::PropertyRunes)?,
                ),
                None => vec![],
            },
            range,
            reload: get_field_from_json(&get_field_from_json(&system_json, "reload"), "value")
                .as_str()
                .map(|x| x.to_string()),
            to_hit_bonus: get_field_from_json(&hit_bonus_json, "value").as_i64(),
            weapon_type: weapon_type.to_uppercase(),
            damage_data: WeaponDamageData::init_vec_from_json(&system_json),
            attack_effects: get_field_from_json(
                &get_field_from_json(&system_json, "attackEffects"),
                "value",
            )
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct WeaponDamageData {
    /// e.g. `"persistent"` or `None` for the base damage instance itself.
    pub category: Option<String>,
    pub dmg_type: Option<String>,
    pub dice_data: ParsedDiceFormula,
}

impl TryFrom<&Value> for WeaponDamageData {
    type Error = BybeItemParsingError;

    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let dmg = json
            .get("damage")
            .and_then(|x| x.as_str())
            .ok_or(BybeItemParsingError::InvalidItemType)?;

        let parsed = parse_dice_formula(dmg).ok_or(BybeItemParsingError::DamageData)?;

        Ok(Self {
            category: None,
            dmg_type: get_field_from_json(json, "damageType")
                .as_str()
                .map(|x| x.to_string()),
            dice_data: parsed,
        })
    }
}

impl WeaponDamageData {
    pub fn init_vec_from_json(json: &Value) -> Vec<WeaponDamageData> {
        let result_vec = match json.get("damage") {
            Some(x) => {
                let mut damages = vec![WeaponDamageData {
                    category: None,
                    dmg_type: get_field_from_json(x, "damageType")
                        .as_str()
                        .map(|x| x.to_string()),
                    dice_data: ParsedDiceFormula {
                        n_of_dice: get_field_from_json(x, "dice").as_i64(),
                        die_size: Some(
                            get_field_from_json(x, "die")
                                .as_str()
                                .and_then(|x| x.replace("d", "").parse::<i64>().ok())
                                .unwrap_or(1),
                        ),
                        bonus_dmg: 0,
                        unresolved_terms: vec![],
                    },
                }];
                if let Some(persistent) = x.get("persistent")
                    && let Some(persistent_dmg) = Self::extract_persistent_dmg_from_json(persistent)
                {
                    damages.push(persistent_dmg);
                }
                damages
            }
            None => {
                let mut weapon_dmg_vec = Vec::new();
                if let Some(key_value_map) = json.get("damageRolls").and_then(|x| x.as_object()) {
                    for (_key, value) in key_value_map {
                        if let Ok(dmg_data) = Self::try_from(value) {
                            weapon_dmg_vec.push(dmg_data);
                        }
                    }
                }
                weapon_dmg_vec
            }
        };
        if result_vec.is_empty() {
            vec![WeaponDamageData {
                category: None,
                dmg_type: None,
                dice_data: ParsedDiceFormula::default(),
            }]
        } else {
            result_vec
        }
    }
    fn extract_persistent_dmg_from_json(persistent: &Value) -> Option<WeaponDamageData> {
        let number = persistent.get("number").and_then(Value::as_i64)?;
        let faces = persistent.get("faces").and_then(Value::as_i64);
        Some(WeaponDamageData {
            category: Some("persistent".to_string()),
            dmg_type: persistent
                .get("type")
                .and_then(Value::as_str)
                .map(String::from),
            dice_data: ParsedDiceFormula {
                n_of_dice: faces.map(|_| number),
                die_size: faces,
                bonus_dmg: if faces.is_none() { number } else { 0 },
                unresolved_terms: vec![],
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct BybeShield {
    pub item_core: BybeItem,
    pub n_of_reinforcing_runes: i64,
    pub ac_bonus: i64,
    pub speed_penalty: i64,
}

impl TryFrom<(&Value, bool)> for BybeShield {
    type Error = BybeItemParsingError;

    fn try_from(args: (&Value, bool)) -> Result<Self, Self::Error> {
        let (json, is_derived) = args;
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .map(|x| x.to_ascii_lowercase())
            .ok_or(BybeItemParsingError::MissingItemType)?;
        if !item_type.eq("shield") {
            return Err(BybeItemParsingError::InvalidItemType);
        }

        let system_json = get_field_from_json(json, "system");
        let item_core = BybeItem::from((SourceItem::try_from(json)?, is_derived));
        let specific_json = get_field_from_json(json, "specific");

        Ok(BybeShield {
            item_core,
            n_of_reinforcing_runes: get_field_from_json(
                &get_field_from_json(&specific_json, "runes"),
                "reinforcing",
            )
            .as_i64()
            .unwrap_or(0),
            ac_bonus: get_field_from_json(&system_json, "acBonus")
                .as_i64()
                .unwrap_or(0),
            speed_penalty: get_field_from_json(&system_json, "speedPenalty")
                .as_i64()
                .unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod weapon_damage_data_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_flat_persistent_damage_alongside_base_damage() {
        // Real Alchemist's Fire (Moderate) shape: 2d8 fire damage plus 2 persistent fire
        let system = json!({
            "damage": {
                "damageType": "fire",
                "dice": 2,
                "die": "d8",
                "persistent": { "faces": null, "number": 2, "type": "fire" }
            }
        });
        let damages = WeaponDamageData::init_vec_from_json(&system);
        assert_eq!(2, damages.len());
        assert_eq!(None, damages[0].category);
        assert_eq!(Some("persistent".to_string()), damages[1].category);
        assert_eq!(Some("fire".to_string()), damages[1].dmg_type);
        assert_eq!(None, damages[1].dice_data.n_of_dice);
        assert_eq!(None, damages[1].dice_data.die_size);
        assert_eq!(2, damages[1].dice_data.bonus_dmg);
    }

    #[test]
    fn extracts_dice_based_persistent_damage() {
        // Real Infused Acid Flask (Moderate) shape: 1 acid damage plus 2d6 persistent acid
        let system = json!({
            "damage": {
                "damageType": "acid",
                "dice": 1,
                "die": "",
                "persistent": { "faces": 6, "number": 2, "type": "acid" }
            }
        });
        let damages = WeaponDamageData::init_vec_from_json(&system);
        assert_eq!(2, damages.len());
        assert_eq!(Some("persistent".to_string()), damages[1].category);
        assert_eq!(Some(2), damages[1].dice_data.n_of_dice);
        assert_eq!(Some(6), damages[1].dice_data.die_size);
        assert_eq!(0, damages[1].dice_data.bonus_dmg);
    }

    #[test]
    fn ignores_absent_persistent_field() {
        let system = json!({
            "damage": { "damageType": "slashing", "dice": 1, "die": "d6" }
        });
        let damages = WeaponDamageData::init_vec_from_json(&system);
        assert_eq!(1, damages.len());
        assert_eq!(None, damages[0].category);
    }

    #[test]
    fn ignores_empty_persistent_object() {
        let system = json!({
            "damage": {
                "damageType": "slashing",
                "dice": 1,
                "die": "d6",
                "persistent": { "faces": null, "number": null, "type": null }
            }
        });
        let damages = WeaponDamageData::init_vec_from_json(&system);
        assert_eq!(1, damages.len());
    }
}
