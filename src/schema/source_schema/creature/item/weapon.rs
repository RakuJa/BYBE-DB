use crate::schema::json_utils;
use crate::schema::publication_info::PublicationInfo;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,

    pub base_weapon: String,

    pub to_hit_bonus: i64,
    // We can ignore this, I think it's always
    // initialized to zero, as such it's more of
    // a modifier that gets applied to a creature
    // pub to_dmg_bonus: Option<i32>,
    pub bulk: i64,
    pub category: String,

    pub damage: Option<WeaponDamageData>,

    pub carry_type: Option<String>,
    pub hands_held: Option<i64>,
    pub invested: Option<bool>,
    pub weapon_group: String,

    pub hardness: Option<i64>,
    pub hp_max: Option<i64>,
    pub hp_curr: Option<i64>,

    pub level: Option<i64>,

    // skipping material and price
    pub publication_info: PublicationInfo,

    pub quantity: Option<i64>,
    pub range: Option<String>,
    pub reload: Option<String>,

    // I don't care about runes rn, maybe will add them later
    // prob useful for runes like reach or shape-shifting
    pub size: String,
    pub traits: WeaponTraits,

    pub usage: String,
    pub weapon_type: String,
}

impl Weapon {
    pub fn init_from_json(json: Value) -> Weapon {
        let system_json = json_utils::get_field_from_json(&json, "system");

        let hit_bonus_json = json_utils::get_field_from_json(&system_json, "bonus");
        let bulk_json = json_utils::get_field_from_json(&system_json, "bulk");
        let equipped_json = json_utils::get_field_from_json(&system_json, "equipped");
        let hp_json = json_utils::get_field_from_json(&system_json, "hp");
        let level_json = json_utils::get_field_from_json(&system_json, "level");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        let reload_json = json_utils::get_field_from_json(&system_json, "reload");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        let usage_json = json_utils::get_field_from_json(&system_json, "usage");
        Weapon {
            name: json_utils::get_field_from_json(&json, "name")
                .as_str()
                .unwrap()
                .to_string(),
            base_weapon: json_utils::get_field_from_json(&system_json, "baseItem")
                .as_str()
                .unwrap()
                .to_string(),
            to_hit_bonus: json_utils::get_field_from_json(&hit_bonus_json, "value")
                .as_i64()
                .unwrap(),
            bulk: json_utils::get_field_from_json(&bulk_json, "value")
                .as_i64()
                .unwrap_or(0),
            category: json_utils::get_field_from_json(&system_json, "simple")
                .as_str()
                .unwrap()
                .to_string(),
            damage: WeaponDamageData::init_from_json(&system_json),
            carry_type: equipped_json
                .get("carryType")
                .map(|x| x.as_str().unwrap().to_string()),
            hands_held: equipped_json.get("handsHeld").map(|x| x.as_i64().unwrap()),
            invested: equipped_json.get("invested").map(|x| x.as_bool().unwrap()),
            weapon_group: json_utils::get_field_from_json(&system_json, "group")
                .as_str()
                .unwrap()
                .to_string(),
            hardness: system_json.get("hardness").map(|x| x.as_i64().unwrap()),
            hp_max: hp_json.get("max").map(|x| x.as_i64().unwrap()),
            hp_curr: hp_json.get("value").map(|x| x.as_i64().unwrap()),
            level: level_json.get("value").map(|x| x.as_i64().unwrap()),
            publication_info: PublicationInfo::init_from_json(&publication_json),
            quantity: system_json.get("quantity").map(|x| x.as_i64().unwrap()),
            range: equipped_json
                .get("range")
                .map(|x| x.as_str().unwrap().to_string()),
            reload: reload_json
                .get("value")
                .map(|x| x.as_str().unwrap().to_string()),
            size: json_utils::get_field_from_json(&system_json, "size")
                .as_str()
                .unwrap()
                .to_string(),
            traits: WeaponTraits::init_from_json(&traits_json),
            usage: json_utils::get_field_from_json(&usage_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            weapon_type: json_utils::get_field_from_json(&json, "type")
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeaponTraits {
    pub rarity: String,
    pub traits: Vec<String>,
}

impl WeaponTraits {
    pub fn init_from_json(json: &Value) -> WeaponTraits {
        WeaponTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .unwrap()
                .to_string(),
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "value")
                    .as_array()
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
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
                    .as_object()
                    .and_then(|x| x.keys().next())
                    .and_then(|key| json.get(key));
                json_obj.and_then(|x| {
                    let dmg = x.get("damage")?.as_str()?;
                    let (n_dices, dmg_data) = dmg.split_once('d')?;
                    let (die, bonus_dmg) = dmg_data.split_once('+')?;
                    Some(WeaponDamageData {
                        dmg_type: json_utils::get_field_from_json(x, "damageType")
                            .as_str()?
                            .to_string(),
                        n_of_dices: n_dices.parse().ok()?,
                        bonus_dmg: bonus_dmg.parse().ok()?,
                        die_size: format!("d{}", die),
                    })
                })
            }
            Some(x) => Some(WeaponDamageData {
                dmg_type: json_utils::get_field_from_json(x, "damageType")
                    .as_str()
                    .unwrap()
                    .to_string(),
                n_of_dices: json_utils::get_field_from_json(x, "dice").as_i64().unwrap(),
                die_size: json_utils::get_field_from_json(x, "die")
                    .as_str()
                    .unwrap()
                    .to_string(),
                bonus_dmg: 0,
            }),
        }
    }
}
