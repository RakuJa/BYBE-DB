use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::source_schema::item::source_item::{SourceItem, SourceItemParsingError};
use crate::utils::json_utils;
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;

#[derive(Clone)]
pub struct BybeItem {
    pub name: String,
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
        if !(item_type.eq("equipment") | item_type.eq("consumable")) {
            return Err(BybeItemParsingError::UnsupportedItemType);
        }
        Ok(Self::from((SourceItem::try_from(json)?, is_derived)))
    }
}

impl From<(SourceItem, bool)> for BybeItem {
    fn from(args: (SourceItem, bool)) -> Self {
        let (source_item, is_derived) = args;
        BybeItem {
            name: source_item.name,
            bulk: source_item.bulk,
            quantity: source_item.quantity,
            base_item: source_item.base_item,
            category: source_item.category,
            description: source_item.description,
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BybeWeapon {
    pub item_core: BybeItem,
    pub to_hit_bonus: Option<i64>,
    pub damage_data: Vec<WeaponDamageData>,
    pub splash_dmg: Option<i64>,
    pub n_of_potency_runes: i64,
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    pub range: Option<i64>,
    pub reload: Option<String>,
    pub weapon_type: String,
}

impl TryFrom<(&Value, bool)> for BybeWeapon {
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

        let weapon_type = if tmp_weapon_type.is_empty() {
            String::from(if item_type.eq("melee") {
                "melee"
            } else {
                "generic"
            })
        } else {
            tmp_weapon_type
        };

        Ok(BybeWeapon {
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
            range: get_field_from_json(&system_json, "range").as_i64(),
            reload: get_field_from_json(&get_field_from_json(&system_json, "reload"), "value")
                .as_str()
                .map(|x| x.to_string()),
            to_hit_bonus: get_field_from_json(&hit_bonus_json, "value").as_i64(),
            weapon_type: weapon_type.to_uppercase(),
            damage_data: WeaponDamageData::init_vec_from_json(&system_json),
        })
    }
}

#[derive(Clone)]
pub struct WeaponDamageData {
    pub dmg_type: Option<String>,
    pub n_of_dice: Option<i64>,
    pub die_size: Option<i64>,
    pub bonus_dmg: i64,
}

impl TryFrom<&Value> for WeaponDamageData {
    type Error = BybeItemParsingError;

    /// parses a single item of a "DamageRolls"-like json
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        if let Some(dmg) = json.get("damage").and_then(|x| x.as_str()) {
            let (n_dices, dmg_data) = if let Some(x) = dmg.split_once('d') {
                x
            } else {
                // we have a single number, so we put in dmg that will put it in the die size
                // aka 1 => 1d1 | 3 => 3d1
                (dmg, "1")
            };
            let (die, bonus_dmg) = if dmg_data.contains('+') {
                let (die, dmg) = dmg_data
                    .split_once('+')
                    .ok_or(BybeItemParsingError::DamageData)?;
                (die, dmg.to_string())
            } else if dmg_data.contains('-') {
                let (die, dmg) = dmg_data
                    .split_once('-')
                    .ok_or(BybeItemParsingError::DamageData)?;
                (die, format!("-{dmg}"))
            } else {
                (dmg_data, "".to_string())
            };
            Ok(WeaponDamageData {
                dmg_type: get_field_from_json(json, "damageType")
                    .as_str()
                    .map(|x| x.to_string()),
                n_of_dice: n_dices.trim().parse::<i64>().ok(),
                bonus_dmg: bonus_dmg
                    .parse()
                    .map(|x: String| x.trim().parse::<i64>().unwrap_or(0))
                    .unwrap_or(0),
                die_size: die.trim().parse::<i64>().ok(),
            })
        } else {
            Err(BybeItemParsingError::InvalidItemType)
        }
    }
}

impl WeaponDamageData {
    pub fn init_vec_from_json(json: &Value) -> Vec<WeaponDamageData> {
        let result_vec = match json.get("damage") {
            Some(x) => {
                vec![WeaponDamageData {
                    dmg_type: get_field_from_json(x, "damageType")
                        .as_str()
                        .map(|x| x.to_string()),
                    n_of_dice: get_field_from_json(x, "dice").as_i64(),
                    die_size: Some(
                        get_field_from_json(x, "die")
                            .as_str()
                            .and_then(|x| x.replace("d", "").parse::<i64>().ok())
                            .unwrap_or(1),
                    ),
                    bonus_dmg: 0,
                }]
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
                dmg_type: None,
                n_of_dice: None,
                die_size: None,
                bonus_dmg: get_field_from_json(&get_field_from_json(json, "bonusDamage"), "value")
                    .as_i64()
                    .unwrap_or(0),
            }]
        } else {
            result_vec
        }
    }
}

#[derive(Clone)]
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
