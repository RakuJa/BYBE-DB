use crate::schema::source_schema::creature::abilities::{AbilityParsingError, RawAbilities};
use crate::schema::source_schema::creature::attributes::{AttributeParsingError, RawAttributes};
use crate::schema::source_schema::creature::details::{DetailsParsingError, RawDetails};
use crate::schema::source_schema::creature::item::items::{
    CreatureItemParsingError, ItemLinkedToCreature,
};
use crate::schema::source_schema::creature::perception::{PerceptionParsingError, RawPerception};
use crate::schema::source_schema::creature::resources::RawResource;
use crate::schema::source_schema::creature::saves::{RawSaves, SaveParsingError};
use crate::schema::source_schema::traits::{RawTraits, TraitParsingError};
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

pub struct SourceCreature {
    pub name: String,
    pub creature_type: String,

    pub abilities: RawAbilities,
    pub attributes: RawAttributes,
    pub details: RawDetails,
    pub initiative_ability: String,
    pub perception: RawPerception,
    pub resource: RawResource,
    pub saves: RawSaves,
    pub traits: RawTraits,
    pub items: ItemLinkedToCreature,
}

#[derive(Debug, Error)]
pub enum SourceCreatureParsingError {
    #[error("Duplicated creature, elite and weak are calculated in runtime")]
    DuplicatedCreature,
    #[error("Creature type could not be parsed")]
    CreatureTypeFormat,
    #[error("Invalid creature type")]
    InvalidCreatureType,
    #[error("Mandatory name field could not be parsed")]
    NameFormat,
    #[error("Initiative ability could not be parsed")]
    InitiativeAbilityFormat,
    #[error("Source item could not be parsed")]
    ResistanceError(#[from] AttributeParsingError),
    #[error("Item related to creature could not be parsed")]
    CreatureItemError(#[from] CreatureItemParsingError),
    #[error("Creature details could not be parsed")]
    CreatureDetailError(#[from] DetailsParsingError),
    #[error("Creature ability could not be parsed")]
    CreatureAbilityError(#[from] AbilityParsingError),
    #[error("Creature perception could not be parsed")]
    CreaturePerceptionError(#[from] PerceptionParsingError),
    #[error("Creature traits could not be parsed")]
    CreatureTraitParsingError(#[from] TraitParsingError),
    #[error("Creature saves could not be parsed")]
    CreatureSaveParsingError(#[from] SaveParsingError),
}

impl TryFrom<&Value> for SourceCreature {
    type Error = SourceCreatureParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let creature_type = json_utils::get_field_from_json(json, "type")
            .as_str()
            .map(String::from)
            .ok_or(SourceCreatureParsingError::CreatureTypeFormat)?;
        if !creature_type.eq_ignore_ascii_case("npc") {
            return Err(SourceCreatureParsingError::InvalidCreatureType);
        }

        let system_json = json_utils::get_field_from_json(json, "system");
        let abilities_json = json_utils::get_field_from_json(&system_json, "abilities");
        let attributes_json = json_utils::get_field_from_json(&system_json, "attributes");
        let details_json = json_utils::get_field_from_json(&system_json, "details");
        let initiative_json = json_utils::get_field_from_json(&system_json, "initiative");
        let perception_json = json_utils::get_field_from_json(&system_json, "perception");
        let resource_json = json_utils::get_field_from_json(&system_json, "resources");
        let saves_json = json_utils::get_field_from_json(&system_json, "saves");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        let items_json = json_utils::get_field_from_json(json, "items");

        let name = json_utils::get_field_from_json(json, "name")
            .as_str()
            .map(String::from)
            .ok_or(SourceCreatureParsingError::NameFormat)?;
        if name.to_uppercase().starts_with("ELITE ") || name.to_uppercase().starts_with("WEAK ") {
            return Err(SourceCreatureParsingError::DuplicatedCreature);
        }
        Ok(SourceCreature {
            name,
            creature_type,

            abilities: RawAbilities::try_from(&abilities_json)?,
            attributes: RawAttributes::try_from(&attributes_json)?,
            details: RawDetails::try_from(&details_json)?,
            initiative_ability: json_utils::get_field_from_json(&initiative_json, "statistic")
                .as_str()
                .map(String::from)
                .ok_or(SourceCreatureParsingError::InitiativeAbilityFormat)?,
            perception: RawPerception::try_from(&perception_json)?,
            resource: RawResource::from(&resource_json),
            saves: RawSaves::try_from(&saves_json)?,
            traits: RawTraits::try_from(&traits_json)?,
            items: ItemLinkedToCreature::try_from(&items_json)?,
        })
    }
}
