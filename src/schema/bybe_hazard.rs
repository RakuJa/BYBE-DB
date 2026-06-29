use crate::schema::bybe_item::BybeWeapon;
use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::resistance::Resistance;
use crate::schema::source_schema::common::saves::RawSaves;
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::hazard::source_hazard::{SourceHazard, SourceHazardParsingError};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct BybeHazard {
    pub name: String,
    pub foundry_id: String,

    // Attributes
    pub ac_value: Option<i64>,
    pub hardness: i64,
    pub has_health: bool,
    pub hp: Option<i64>,
    pub hp_details: Option<String>,
    pub stealth: i64,
    pub stealth_detail: Description,
    pub immunities: Vec<String>,
    pub resistances: Vec<Resistance>,
    pub weaknesses: HashMap<String, i64>,

    // Details
    pub description: Description,
    pub disable_description: Description,
    pub reset_description: Description,
    pub routine_description: Description,
    pub is_complex: bool,
    pub level: i64,
    pub publication_info: PublicationInfo,

    pub saves: RawSaves,
    pub weapons: Vec<BybeWeapon>,
    pub actions: Vec<Action>,
    pub rarity: RarityEnum,
    pub size: SizeEnum,
    pub traits: Vec<String>,
}

#[derive(Debug, Error)]
pub enum BybeHazardParsingError {
    #[error("Source hazard could not be parsed")]
    SourceHazardError(#[from] SourceHazardParsingError),
}

impl TryFrom<&Value> for BybeHazard {
    type Error = BybeHazardParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(Self::from(SourceHazard::try_from(json)?))
    }
}

impl From<SourceHazard> for BybeHazard {
    fn from(source_hz: SourceHazard) -> Self {
        let weapons = source_hz
            .items
            .weapon_list
            .into_iter()
            .map(|x| {
                BybeWeapon::builder()
                    .creature_actions(&source_hz.items.action_list)
                    .source_weapon(x)
                    .build()
            })
            .collect();

        if !source_hz.attributes.speed.is_empty() {
            println!("not empty speed");
        }

        Self {
            name: source_hz.name,
            foundry_id: source_hz.foundry_id,
            actions: source_hz.items.action_list,
            ac_value: source_hz.attributes.ac,
            hardness: source_hz.attributes.hardness,
            has_health: source_hz.attributes.has_health,
            hp: source_hz.attributes.hp_values.as_ref().map(|x| x.hp),
            hp_details: source_hz
                .attributes
                .hp_values
                .as_ref()
                .map(|x| x.hp_details.clone().unwrap()),
            stealth: source_hz.attributes.stealth,
            stealth_detail: source_hz.attributes.stealth_detail,
            resistances: source_hz.attributes.resistances,
            immunities: source_hz.attributes.immunities,
            description: source_hz.description,
            disable_description: source_hz.disable_description,
            reset_description: source_hz.reset_description,
            routine_description: source_hz.routine_description,
            is_complex: source_hz.is_complex,
            level: source_hz.level,
            publication_info: source_hz.publication_info,
            saves: source_hz.saves,
            rarity: source_hz
                .traits
                .rarity
                .as_str()
                .parse()
                .unwrap_or(RarityEnum::Common),
            size: source_hz
                .traits
                .size
                .as_str()
                .parse()
                .unwrap_or(SizeEnum::Medium),
            traits: source_hz.traits.traits,
            weapons,
            weaknesses: source_hz.attributes.weakness,
        }
    }
}
