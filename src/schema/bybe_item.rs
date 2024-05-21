use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::source_schema::item::source_item::SourceItem;
use serde_json::Value;

pub struct BybeItem {
    pub name: String,
    pub bulk: f64,
    pub category: Option<String>,
    pub description: String,
    pub hardness: i64,
    pub hp: i64,
    pub level: i64,
    pub price: i64, // in cp,
    pub usage: String,
    pub item_type: String,
    pub material_grade: Option<String>,
    pub material_type: Option<String>,
    pub number_of_uses: Option<i64>, // for consumables, for equip set as null.

    // source details (title, license, remastered)
    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub rarity: RarityEnum,
    pub size: SizeEnum,
    pub traits: Vec<String>,
}

impl BybeItem {
    pub fn init_from_json(json: &Value) -> Option<BybeItem> {
        Some(Self::init_from_source_item(SourceItem::init_from_json(
            json,
        )?))
    }
    pub fn init_from_source_item(source_item: SourceItem) -> BybeItem {
        BybeItem {
            name: source_item.name,
            bulk: source_item.bulk,
            category: source_item.category,
            description: source_item.description,
            hardness: source_item.hardness,
            hp: source_item.hp_values.hp,
            level: source_item.level,
            price: source_item.price.to_cp(),
            usage: source_item.usage,
            item_type: source_item.item_type.to_uppercase(),
            material_grade: source_item.material.grade,
            material_type: source_item.material.m_type,
            number_of_uses: source_item.uses,
            license: source_item.publication_info.license,
            remaster: source_item.publication_info.remastered,
            source: source_item.publication_info.source,
            rarity: source_item
                .traits
                .rarity
                .as_str()
                .parse()
                .unwrap_or(RarityEnum::Common),
            size: source_item
                .traits
                .size
                .as_str()
                .parse()
                .unwrap_or(SizeEnum::Medium),
            traits: source_item.traits.traits,
        }
    }
}
