use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::source_schema::item::source_item::SourceItem;
use crate::utils::json_utils;
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;

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
}

impl BybeItem {
    pub fn init_from_json(json: &Value) -> Option<BybeItem> {
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .unwrap()
            .to_string()
            .to_ascii_lowercase();
        if !(item_type.eq("equipment") | item_type.eq("consumable")) {
            return None;
        }
        Some(Self::init_from_source_item(SourceItem::init_from_json(
            json,
        )?))
    }
    pub fn init_from_source_item(source_item: SourceItem) -> BybeItem {
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
        }
    }
}

pub struct BybeArmor {
    pub item_core: BybeItem,
    pub ac_bonus: i64,
    pub check_penalty: i64,
    pub dex_cap: i64,
    pub n_of_potency_runes: i64,
    pub property_runes: Vec<String>,
    pub n_of_resilient_runes: i64,
    pub speed_penalty: i64,
    pub strength_required: i64,
}

impl BybeArmor {
    pub fn init_from_json(json: &Value) -> Option<BybeArmor> {
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .unwrap()
            .to_string()
            .to_ascii_lowercase();
        if !item_type.eq("armor") {
            return None;
        }
        let system_json = get_field_from_json(json, "system");
        let item_core = BybeItem::init_from_source_item(SourceItem::init_from_json(json)?);
        let rune_json = get_field_from_json(&system_json, "runes");
        Some(BybeArmor {
            item_core,
            ac_bonus: get_field_from_json(&system_json, "acBonus")
                .as_i64()
                .unwrap(),
            check_penalty: get_field_from_json(&system_json, "checkPenalty")
                .as_i64()
                .unwrap(),
            dex_cap: get_field_from_json(&system_json, "dexCap")
                .as_i64()
                .unwrap(),
            n_of_potency_runes: get_field_from_json(&rune_json, "potency").as_i64().unwrap(),
            property_runes: json_utils::from_json_vec_of_str_to_vec_of_str(
                get_field_from_json(&rune_json, "property")
                    .as_array()
                    .unwrap(),
            ),
            n_of_resilient_runes: get_field_from_json(&rune_json, "resilient")
                .as_i64()
                .unwrap(),
            speed_penalty: get_field_from_json(&system_json, "speedPenalty")
                .as_i64()
                .unwrap(),
            strength_required: get_field_from_json(&system_json, "strength")
                .as_i64()
                .unwrap_or(0),
        })
    }
}

#[derive(Clone)]
pub struct BybeWeapon {
    pub item_core: BybeItem,
    pub bonus_dmg: i64,
    pub to_hit_bonus: Option<i64>,
    pub dmg_type: Option<String>,
    pub number_of_dice: Option<i64>,
    pub die_size: Option<String>,
    pub splash_dmg: Option<i64>,
    pub n_of_potency_runes: i64,
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    pub range: Option<i64>,
    pub reload: Option<String>,
    pub weapon_type: String,
}

impl BybeWeapon {
    pub fn init_from_json(json: &Value) -> Option<BybeWeapon> {
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .unwrap()
            .to_string()
            .to_ascii_lowercase();
        if !(item_type.eq("weapon") | item_type.eq("melee")) {
            return None;
        }
        let system_json = get_field_from_json(json, "system");
        let item_core = BybeItem::init_from_source_item(SourceItem::init_from_json(json)?);
        let runes_data = get_field_from_json(&system_json, "runes");
        let (die_size, number_of_dice, dmg_type) =
            match WeaponDamageData::init_from_json(&get_field_from_json(&system_json, "damage")) {
                Some(x) => (Some(x.die_size), Some(x.n_of_dices), Some(x.dmg_type)),
                None => (None, None, None),
            };
        let hit_bonus_json = get_field_from_json(&system_json, "bonus");
        let wp_type_json = get_field_from_json(&system_json, "weaponType");

        Some(BybeWeapon {
            item_core,
            bonus_dmg: get_field_from_json(
                &get_field_from_json(&system_json, "bonusDamage"),
                "value",
            )
            .as_i64()
            .unwrap_or(0),
            die_size,
            number_of_dice,
            dmg_type,
            splash_dmg: get_field_from_json(
                &get_field_from_json(&system_json, "splashDamage"),
                "value",
            )
            .as_i64(),
            n_of_potency_runes: get_field_from_json(&runes_data, "potency")
                .as_i64()
                .unwrap_or(0),
            n_of_striking_runes: get_field_from_json(&runes_data, "striking")
                .as_i64()
                .unwrap_or(0),
            property_runes: match runes_data.get("property") {
                Some(x) => json_utils::from_json_vec_of_str_to_vec_of_str(x.as_array().unwrap()),
                None => vec![],
            },
            range: get_field_from_json(&system_json, "range").as_i64(),
            reload: get_field_from_json(&get_field_from_json(&system_json, "reload"), "value")
                .as_str()
                .map(|x| x.to_string()),
            to_hit_bonus: get_field_from_json(&hit_bonus_json, "value").as_i64(),
            weapon_type: get_field_from_json(&wp_type_json, "value")
                .as_str()
                .unwrap_or("melee")
                .to_string()
                .to_uppercase(),
        })
    }
}

pub struct WeaponDamageData {
    pub dmg_type: String,
    pub n_of_dices: i64,
    pub die_size: String,
    pub bonus_dmg: i64,
}

impl WeaponDamageData {
    pub fn init_from_json(json: &Value) -> Option<WeaponDamageData> {
        match json.get("damage") {
            None => {
                let json_obj = json
                    .get("damageRolls")
                    .and_then(|x| x.as_object())
                    .and_then(|json_map| json_map.values().next());
                json_obj.and_then(|x| {
                    let dmg = x.get("damage")?.as_str()?;
                    let (n_dices, dmg_data) = dmg.split_once('d')?;
                    let (die, bonus_dmg) = dmg_data.split_once('+')?;
                    Some(WeaponDamageData {
                        dmg_type: get_field_from_json(x, "damageType").as_str()?.to_string(),
                        n_of_dices: n_dices.parse().ok()?,
                        bonus_dmg: bonus_dmg.parse().ok()?,
                        die_size: format!("d{}", die),
                    })
                })
            }
            Some(x) => Some(WeaponDamageData {
                dmg_type: get_field_from_json(x, "damageType")
                    .as_str()
                    .unwrap()
                    .to_string(),
                n_of_dices: get_field_from_json(x, "dice").as_i64().unwrap(),
                die_size: get_field_from_json(x, "die").as_str().unwrap().to_string(),
                bonus_dmg: 0,
            }),
        }
    }
}
