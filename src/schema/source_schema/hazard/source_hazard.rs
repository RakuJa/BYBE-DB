use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::hp_values::HpParsingError;
use crate::schema::source_schema::common::rarity_size_traits::{
    RaritySizeTraits, TraitParsingError,
};
use crate::schema::source_schema::common::saves::{RawSaves, SaveParsingError};
use crate::schema::source_schema::creature::item::action::ActionParsingError;
use crate::schema::source_schema::creature::item::items::{
    EntityItemParsingError, ItemLinkedToEntity,
};
use crate::schema::source_schema::hazard::attributes::{AttributeParsingError, RawAttributes};
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;

pub struct SourceHazard {
    pub name: String,
    pub foundry_id: String,

    pub attributes: RawAttributes,

    pub creature_type: Option<String>,
    // Details
    pub description: Description,
    pub disable_description: Description,
    pub is_complex: bool,
    pub level: i64,
    pub publication_info: PublicationInfo,
    pub reset_description: Description,
    pub routine_description: Description,

    pub saves: RawSaves,
    pub items: ItemLinkedToEntity,
    pub traits: RaritySizeTraits,
}

#[derive(Debug, Error)]
pub enum SourceHazardParsingError {
    #[error("Source id missing")]
    IdMissing,
    #[error("Mandatory Name field is missing from json")]
    NameFieldMissing,
    #[error("Publication info could not be parsed")]
    PublicationError(#[from] PublicationParsingError),
    #[error("Hp Value could not be parsed")]
    HpError(#[from] HpParsingError),
    #[error("Trait values could not be parsed")]
    TraitError(#[from] TraitParsingError),
    #[error("Could not parse action related to hazard")]
    HazardActionError(#[from] ActionParsingError),
    #[error("Could not parse saving stats related to hazard")]
    HazardSavesError(#[from] SaveParsingError),
    #[error("Hazard type could not be parsed")]
    HazardTypeFormat,
    #[error("Invalid hazard type")]
    InvalidHazardType,
    #[error("Item related to hazard could not be parsed")]
    HazardItemError(#[from] EntityItemParsingError),
    #[error("Attribute could not be parsed")]
    AttributeError(#[from] AttributeParsingError),
}

impl TryFrom<&Value> for SourceHazard {
    type Error = SourceHazardParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let creature_type = get_field_from_json(json, "type")
            .as_str()
            .map(String::from)
            .ok_or(SourceHazardParsingError::HazardTypeFormat)?;

        if !creature_type.eq_ignore_ascii_case("hazard") {
            return Err(SourceHazardParsingError::InvalidHazardType);
        }

        let foundry_id = get_field_from_json(json, "_id")
            .as_str()
            .map(String::from)
            .ok_or(SourceHazardParsingError::IdMissing)
            .unwrap();
        let name = get_field_from_json(json, "name")
            .as_str()
            .map(|x| x.to_string())
            .ok_or(SourceHazardParsingError::NameFieldMissing)
            .unwrap();

        let system_json = get_field_from_json(json, "system");
        let items_json = get_field_from_json(json, "items");

        let attributes_json = get_field_from_json(&system_json, "attributes");
        let details_json = get_field_from_json(&system_json, "details");
        let saves_json = get_field_from_json(&system_json, "saves");
        let traits_json = get_field_from_json(&system_json, "traits");

        let publication_json = get_field_from_json(&details_json, "publication");

        Ok(Self {
            name,
            foundry_id,
            attributes: RawAttributes::try_from(&attributes_json).unwrap(),
            creature_type: get_field_from_json(&system_json, "creatureType")
                .as_str()
                .map(|x| x.to_string()),
            description: get_field_from_json(&details_json, "description")
                .as_str()
                .map(Description::from)
                .unwrap_or_default(),
            disable_description: get_field_from_json(&details_json, "disable")
                .as_str()
                .map(Description::from)
                .unwrap_or_default(),
            reset_description: get_field_from_json(&details_json, "reset")
                .as_str()
                .map(Description::from)
                .unwrap_or_default(),
            routine_description: get_field_from_json(&details_json, "routine")
                .as_str()
                .map(Description::from)
                .unwrap_or_default(),
            is_complex: get_field_from_json(&details_json, "isComplex")
                .as_bool()
                .unwrap(),
            level: get_field_from_json(&get_field_from_json(&details_json, "level"), "value")
                .as_i64()
                .unwrap_or(0),
            publication_info: PublicationInfo::try_from(&publication_json)?,
            saves: RawSaves::try_from(&saves_json).unwrap(),
            traits: RaritySizeTraits::try_from(&traits_json).unwrap(),
            items: ItemLinkedToEntity::try_from(&items_json).unwrap(),
        })
    }
}
