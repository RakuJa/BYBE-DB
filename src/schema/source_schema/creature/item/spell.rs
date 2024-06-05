use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::creature::item::saving_throw::SavingThrow;
use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub area: Option<Area>,
    pub counteraction: bool,

    pub damage: Vec<RawDamageData>,

    pub saving_throw: Option<SavingThrow>,

    pub sustained: bool,
    pub duration: Option<String>,

    // too complicated atm, it's a vec of struct
    // because there are multiple ways to heighten data
    //pub heightened: Option<HeightenedData>,
    pub level: i64,
    pub range: String,
    pub target: String,
    pub actions: String,

    pub publication_info: PublicationInfo,
    pub traits: SpellTraits,
}

impl Spell {
    pub fn init_from_json(json: &Value) -> Spell {
        let system_json = json_utils::get_field_from_json(json, "system");
        let mut damage_data = Vec::new();
        let mut i = 0;
        let damage_json = json_utils::get_field_from_json(&system_json, "damage");
        loop {
            let dmg_data_json = damage_json.get(i.to_string().as_str());
            if dmg_data_json.is_none() {
                break;
            }
            damage_data.push(RawDamageData::init_from_json(dmg_data_json.unwrap()));
            i += 1;
        }

        let defense_json = json_utils::get_field_from_json(&system_json, "defense");
        let duration_json = json_utils::get_field_from_json(&system_json, "duration");
        let level_json = json_utils::get_field_from_json(&system_json, "level");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        let range_json = json_utils::get_field_from_json(&system_json, "range");
        let target_json = json_utils::get_field_from_json(&system_json, "target");
        let time_json = json_utils::get_field_from_json(&system_json, "time");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        Spell {
            name: json_utils::get_field_from_json(json, "name")
                .as_str()
                .unwrap()
                .to_string(),
            area: match system_json.get("area") {
                Some(x) => Area::init_from_json(x),
                None => None,
            },
            counteraction: json_utils::get_field_from_json(&system_json, "counteraction")
                .as_bool()
                .unwrap_or(false),
            damage: damage_data,
            saving_throw: SavingThrow::init_from_json(&json_utils::get_field_from_json(
                &defense_json,
                "save",
            )),
            sustained: json_utils::get_field_from_json(&duration_json, "sustained")
                .as_bool()
                .unwrap(),
            duration: json_utils::get_field_from_json(&duration_json, "value")
                .as_str()
                .map(|x| x.to_string()),
            //heightened: match system_json.get("heightening") {
            //    Some(x) => Some(HeightenedData::init_from_json(x)),
            //    None => None,
            //},
            level: level_json.get("value").unwrap().as_i64().unwrap(),
            publication_info: PublicationInfo::init_from_json(&publication_json),
            range: json_utils::get_field_from_json(&range_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            target: json_utils::get_field_from_json(&target_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            actions: json_utils::get_field_from_json(&time_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            traits: SpellTraits::init_from_json(&traits_json),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpellTraits {
    pub rarity: String,
    pub traditions: Vec<String>,
    pub traits: Vec<String>,
}

impl SpellTraits {
    pub fn init_from_json(json: &Value) -> SpellTraits {
        SpellTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .unwrap()
                .to_string(),
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
        }
    }
}

/*
#[derive(Debug)]
pub struct HeightenedData {
    pub interval: i64,
    pub heightening_type: String,
    pub damage_dice: Vec<String>,
}

impl HeightenedData {
    pub fn init_from_json(json: &Value) -> HeightenedData {
        let mut damage_dice = Vec::new();
        let mut i = 0;
        let damage_json = json_utils::get_field_from_json(json, "damage");
        loop {
            let dmg_dice_json = damage_json.get(i.to_string().as_str());
            if dmg_dice_json.is_none() {
                break;
            }
            damage_dice.push(dmg_dice_json.unwrap().as_str().unwrap().to_string());
            i += 1;
        }

        HeightenedData {
            interval: json_utils::get_field_from_json(json, "interval")
                .as_i64()
                .unwrap(),
            heightening_type: json_utils::get_field_from_json(json, "type")
                .as_str()
                .unwrap()
                .to_string(),
            damage_dice,
        }
    }
}

 */

#[derive(Debug, Clone)]
pub struct Area {
    pub area_type: String,
    pub area_value: i64,
}

impl Area {
    pub fn init_from_json(json: &Value) -> Option<Area> {
        match json.as_object().is_none() {
            false => Some(Area {
                area_type: json_utils::get_field_from_json(json, "type")
                    .as_str()
                    .unwrap()
                    .to_string(),
                area_value: json_utils::get_field_from_json(json, "value")
                    .as_i64()
                    .unwrap(),
            }),
            true => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawDamageData {
    pub category: Option<String>,
    pub formula: String,
    pub kinds: Vec<String>,
    pub dmg_type: String,
}

impl RawDamageData {
    pub fn init_from_json(json: &Value) -> RawDamageData {
        RawDamageData {
            category: json_utils::get_field_from_json(json, "category")
                .as_str()
                .map(|x| x.to_string()),
            formula: json_utils::get_field_from_json(json, "formula")
                .as_str()
                .unwrap()
                .to_string(),
            kinds: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "kinds")
                    .as_array()
                    .unwrap(),
            ),
            dmg_type: json_utils::get_field_from_json(json, "type")
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
