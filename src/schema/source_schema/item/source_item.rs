use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::description::Description;
use crate::schema::source_schema::hp_values::{HpParsingError, RawHpValues};
use crate::schema::source_schema::item::material::RawMaterial;
use crate::schema::source_schema::price_struct::PriceStruct;
use crate::schema::source_schema::traits::{RawTraits, TraitParsingError};
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;
use thiserror::Error;

pub struct SourceItem {
    pub name: String,
    pub bulk: f64,
    pub quantity: i64,
    pub base_item: Option<String>,
    pub description: String,
    pub hardness: i64,
    pub hp_values: RawHpValues,
    pub level: i64,
    pub price: PriceStruct, // in cp,
    pub publication_info: PublicationInfo,
    pub traits: RawTraits,
    pub usage: Option<String>,
    pub item_type: String,

    pub group: Option<String>,
    pub category: Option<String>,
    pub material: RawMaterial,
    pub uses: Option<i64>, // for consumables, for equip set as null.
}

#[derive(Debug, Error)]
pub enum SourceItemParsingError {
    #[error("Category field is not a string")]
    CategoryFieldMissing,
    #[error("Mandatory Name field is missing from json")]
    NameFieldMissing,
    #[error("Publication info could not be parsed")]
    PublicationError(#[from] PublicationParsingError),
    #[error("Error while parsing type field")]
    TypeFieldError,
    #[error("Hp Value could not be parsed")]
    HpError(#[from] HpParsingError),
    #[error("Trait values could not be parsed")]
    TraitError(#[from] TraitParsingError),
}

impl TryFrom<&Value> for SourceItem {
    type Error = SourceItemParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .map(|x| x.to_ascii_lowercase())
            .ok_or(SourceItemParsingError::TypeFieldError)?;
        let system_json = get_field_from_json(json, "system");
        let hp_json = get_field_from_json(&system_json, "hp");
        let price_json = get_field_from_json(&system_json, "price");
        let traits_json = get_field_from_json(&system_json, "traits");
        let publication_json = get_field_from_json(&system_json, "publication");
        let binding =
            get_field_from_json(&get_field_from_json(&system_json, "specific"), "material");
        let material_json = system_json.get("material").unwrap_or(&binding);
        let uses_json = get_field_from_json(&system_json, "uses");
        let name = get_field_from_json(json, "name")
            .as_str()
            .map(|x| x.to_string())
            .ok_or(SourceItemParsingError::NameFieldMissing)?;

        Ok(SourceItem {
            name,
            bulk: get_field_from_json(&get_field_from_json(&system_json, "bulk"), "value")
                .as_f64()
                .unwrap_or(0.0),
            quantity: get_field_from_json(&system_json, "quantity")
                .as_i64()
                .unwrap_or(1),
            base_item: get_field_from_json(&system_json, "baseItem")
                .as_str()
                .map(|x| x.to_string()),
            description: get_field_from_json(
                &get_field_from_json(&system_json, "description"),
                "value",
            )
            .as_str()
            .map(Description::from)
            .unwrap_or_default()
            .to_string(),
            hardness: get_field_from_json(&system_json, "hardness")
                .as_i64()
                .unwrap_or(0),
            hp_values: RawHpValues::try_from(&hp_json)?,
            level: get_field_from_json(&get_field_from_json(&system_json, "level"), "value")
                .as_i64()
                .unwrap_or(0),
            price: PriceStruct::from(&price_json),
            publication_info: PublicationInfo::try_from(&publication_json)?,
            traits: RawTraits::try_from(&traits_json)?,
            usage: get_field_from_json(&get_field_from_json(&system_json, "usage"), "value")
                .as_str()
                .map(|x| x.to_string()),
            item_type: if item_type.eq("melee") {
                String::from("weapon")
            } else {
                item_type
            },
            group: get_field_from_json(&system_json, "group")
                .as_str()
                .map(|x| x.to_string()),
            category: match system_json.get("category") {
                Some(x) => Some(
                    x.as_str()
                        .ok_or(SourceItemParsingError::CategoryFieldMissing)?
                        .to_string(),
                ),
                _ => None,
            },
            material: RawMaterial::from(material_json),
            uses: get_field_from_json(&uses_json, "max").as_i64(),
        })
    }
}
