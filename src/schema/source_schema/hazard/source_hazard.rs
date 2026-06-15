use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::common::description::Description;
use crate::schema::source_schema::common::hp_values::{HpParsingError, RawHpValues};
use crate::schema::source_schema::common::rarity_size_traits::{
    RaritySizeTraits, TraitParsingError,
};
use crate::schema::source_schema::common::saves::{RawSaves, SaveParsingError};
use crate::schema::source_schema::creature::item::action::{Action, ActionParsingError};
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;
use tracing::warn;
pub struct SourceHazard {
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
    pub status_effect_list: Vec<String>,
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
            .ok_or(SourceHazardParsingError::IdMissing)?;
        let name = get_field_from_json(json, "name")
            .as_str()
            .map(|x| x.to_string())
            .ok_or(SourceHazardParsingError::NameFieldMissing)?;

        let system_json = get_field_from_json(json, "system");
        let items_json = get_field_from_json(json, "items");

        let attributes_json = get_field_from_json(&system_json, "attributes");
        let details_json = get_field_from_json(&system_json, "details");
        let saves_json = get_field_from_json(&system_json, "saves");
        let traits_json = get_field_from_json(&system_json, "traits");

        let hp_json = get_field_from_json(&attributes_json, "hp");

        let publication_json = get_field_from_json(&details_json, "publication");

        let actions: Vec<Action> = items_json
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| {
                let temp = x
                    .get("type")
                    .and_then(|x| x.as_str())
                    .map(|x| x.to_ascii_lowercase());
                if let Some(item_type) = temp
                    && item_type == "action"
                {
                    Action::try_from(x).ok()
                } else {
                    None
                }
            })
            .collect();
        let span = tracing::info_span!("save_parsing", caller = "Source hazard try from");
        let _guard = span.enter();

        let stealth_value =
            get_field_from_json(&get_field_from_json(&attributes_json, "stealth"), "value");
        let stealth_num = if stealth_value.is_null() {
            warn!(
                "Hazard: {:?} does not have a stealth value, setting default to 10",
                name
            );
            10
        } else {
            let val = stealth_value.as_i64().unwrap();
            if val < 10 && val > -10 { val + 10 } else { val }
        };

        Ok(Self {
            name,
            foundry_id,
            actions,
            ac_value: get_field_from_json(&get_field_from_json(&attributes_json, "ac"), "value")
                .as_i64()
                .unwrap_or(0),
            hardness: get_field_from_json(&attributes_json, "hardness")
                .as_i64()
                .unwrap_or(0),
            has_health: get_field_from_json(&attributes_json, "hasHealth")
                .as_bool()
                .unwrap_or(false),
            hp_values: RawHpValues::try_from(&hp_json).unwrap_or_default(),
            stealth: stealth_num,
            stealth_detail: get_field_from_json(
                &get_field_from_json(&attributes_json, "stealth"),
                "details",
            )
            .as_str()
            .map(Description::from)
            .unwrap(),
            creature_type: get_field_from_json(&attributes_json, "creatureType")
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
            saves: RawSaves::try_from(&saves_json)?,
            status_effect_list: vec![],
            traits: RaritySizeTraits::try_from(&traits_json)?,
        })
    }
}
