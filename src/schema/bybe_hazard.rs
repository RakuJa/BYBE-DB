use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::hp_values::RawHpValues;
use crate::schema::source_schema::common::saves::RawSaves;
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::hazard::source_hazard::{SourceHazard, SourceHazardParsingError};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct BybeHazard {
    pub name: String,
    pub foundry_id: String,

    pub actions: Vec<Action>,
    // Attributes
    pub ac_value: i64,
    pub hardness: i64,
    pub has_health: bool,
    pub hp_values: RawHpValues,
    pub stealth: i64,
    pub stealth_detail: Description,

    // Details
    pub description: Description,
    pub disable_description: Description,
    pub reset_description: Description,
    pub routine_description: Description,
    pub is_complex: bool,
    pub level: i64,
    pub publication_info: PublicationInfo,

    pub saves: RawSaves,
    pub status_effect_list: Vec<String>,
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
        Self {
            name: source_hz.name,
            foundry_id: source_hz.foundry_id,
            actions: source_hz.actions,
            ac_value: source_hz.ac_value,
            hardness: source_hz.hardness,
            has_health: source_hz.has_health,
            hp_values: source_hz.hp_values,
            stealth: source_hz.stealth,
            stealth_detail: source_hz.stealth_detail,
            description: source_hz.description,
            disable_description: source_hz.disable_description,
            reset_description: source_hz.reset_description,
            routine_description: source_hz.routine_description,
            is_complex: source_hz.is_complex,
            level: source_hz.level,
            publication_info: source_hz.publication_info,
            saves: source_hz.saves,
            status_effect_list: source_hz.status_effect_list,
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
        }
    }
}
