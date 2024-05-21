use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::hp_values::RawHpValues;
use crate::schema::source_schema::item::material::RawMaterial;
use crate::schema::source_schema::price_struct::PriceStruct;
use crate::schema::source_schema::traits::RawTraits;
use crate::utils::json_utils;
use serde_json::Value;

pub struct SourceItem {
    pub name: String,
    pub bulk: f64,
    pub description: String,
    pub hardness: i64,
    pub hp_values: RawHpValues,
    pub level: i64,
    pub price: PriceStruct, // in cp,
    pub publication_info: PublicationInfo,
    pub traits: RawTraits,
    pub usage: String,
    pub item_type: String,

    pub category: Option<String>,
    pub damage: Option<i64>,
    pub material: RawMaterial,
    pub uses: Option<i64>, // for consumables, for equip set as null.
}

impl SourceItem {
    pub fn init_from_json(json: &Value) -> Option<SourceItem> {
        let item_type = json_utils::get_field_from_json(json, "type")
            .as_str()
            .unwrap()
            .to_string()
            .to_ascii_lowercase();
        if !(item_type.eq("equipment") | item_type.eq("consumable")) {
            return None;
        }
        let system_json = json_utils::get_field_from_json(json, "system");
        let hp_json = json_utils::get_field_from_json(&system_json, "hp");
        let price_json = json_utils::get_field_from_json(&system_json, "price");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        let material_json = json_utils::get_field_from_json(&system_json, "material");
        let uses_json = json_utils::get_field_from_json(&system_json, "uses");
        let name = json_utils::get_field_from_json(json, "name")
            .as_str()
            .unwrap()
            .to_string();
        Some(SourceItem {
            name,
            bulk: json_utils::get_field_from_json(&system_json, "bulk")
                .as_f64()
                .unwrap_or(0.0),
            description: json_utils::get_field_from_json(&system_json, "description")
                .as_str()
                .unwrap_or_default()
                .to_string(),
            hardness: json_utils::get_field_from_json(&system_json, "hardness")
                .as_i64()
                .unwrap(),
            hp_values: RawHpValues::init_from_json(&hp_json),
            level: json_utils::get_field_from_json(&system_json, "level")
                .as_i64()
                .unwrap_or(0),
            price: PriceStruct::init_from_json(&price_json),
            publication_info: PublicationInfo::init_from_json(&publication_json),
            traits: RawTraits::init_from_json(traits_json),
            usage: system_json
                .get("usage")
                .unwrap()
                .get("value")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            item_type,
            category: system_json
                .get("category")
                .map(|x| String::from(x.as_str().unwrap())),
            damage: json_utils::get_field_from_json(&system_json, "damage").as_i64(),
            material: RawMaterial::init_from_json(&material_json),
            uses: json_utils::get_field_from_json(&uses_json, "max").as_i64(),
        })
    }
}
