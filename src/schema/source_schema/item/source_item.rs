use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::description::Description;
use crate::schema::source_schema::hp_values::RawHpValues;
use crate::schema::source_schema::item::material::RawMaterial;
use crate::schema::source_schema::price_struct::PriceStruct;
use crate::schema::source_schema::traits::RawTraits;
use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;

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

impl SourceItem {
    pub fn init_from_json(json: &Value) -> Option<SourceItem> {
        let item_type = get_field_from_json(json, "type")
            .as_str()
            .unwrap()
            .to_string()
            .to_ascii_lowercase();
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
            .unwrap()
            .to_string();
        Some(SourceItem {
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
            .map(Description::initialize)
            .unwrap_or_default()
            .to_string(),
            hardness: get_field_from_json(&system_json, "hardness")
                .as_i64()
                .unwrap_or(0),
            hp_values: RawHpValues::init_from_json(&hp_json),
            level: get_field_from_json(&get_field_from_json(&system_json, "level"), "value")
                .as_i64()
                .unwrap_or(0),
            price: PriceStruct::init_from_json(&price_json),
            publication_info: PublicationInfo::init_from_json(&publication_json),
            traits: RawTraits::init_from_json(traits_json),
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
            category: system_json
                .get("category")
                .map(|x| String::from(x.as_str().unwrap())),
            material: RawMaterial::init_from_json(material_json),
            uses: get_field_from_json(&uses_json, "max").as_i64(),
        })
    }
}
