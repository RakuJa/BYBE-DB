use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::creature::item::saving_throw::SavingThrow;
use crate::schema::source_schema::creature::resistance::ResistanceParserError;
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Spell {
    pub raw_foundry_id: String,
    pub location_id: String,
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
    pub heightened_level: Option<i64>,
    pub range: String,
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
    #[error("Mandatory sustained field is missing from json")]
    Range,
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
    #[error("Damage kind field is missing from json")]
    DamageKind,
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
            name: json_utils::get_field_from_json(json, "name")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::Name)?,
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
            //heightened: match system_json.get("heightening") {
            //    Some(x) => Some(HeightenedData::init_from_json(x)),
            //    None => None,
            //},
            level: level_json
                .get("value")
                .and_then(|x| x.as_i64())
                .ok_or(SpellParsingError::Level)?,
            publication_info: PublicationInfo::try_from(&publication_json)?,
            range: json_utils::get_field_from_json(&range_json, "value")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::Range)?,
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Area {
    pub area_type: String,
    pub area_value: i64,
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
    pub formula: String,
    pub kinds: Vec<String>,
    pub dmg_type: String,
}

impl TryFrom<&Value> for RawDamageData {
    type Error = SpellParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(RawDamageData {
            category: json_utils::get_field_from_json(json, "category")
                .as_str()
                .map(|x| x.to_string()),
            formula: json_utils::get_field_from_json(json, "formula")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::DamageFormula)?,
            kinds: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "kinds")
                    .as_array()
                    .ok_or(SpellParsingError::DamageKind)?,
            ),
            dmg_type: json_utils::get_field_from_json(json, "type")
                .as_str()
                .map(String::from)
                .ok_or(SpellParsingError::DamageType)?,
        })
    }
}
