use crate::schema::foundry_schema::creature::abilities::RawAbilities;
use crate::schema::foundry_schema::creature::attributes::RawAttributes;
use crate::schema::foundry_schema::creature::details::RawDetails;
use crate::schema::foundry_schema::creature::item::items::RawItems;
use crate::schema::foundry_schema::creature::perception::RawPerception;
use crate::schema::foundry_schema::creature::saves::RawSaves;
use crate::schema::foundry_schema::creature::traits::RawTraits;
use crate::schema::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct FoundryCreature {
    pub name: String,
    pub creature_type: String,

    pub abilities: RawAbilities,
    pub attributes: RawAttributes,
    pub details: RawDetails,
    pub initiative_ability: String,
    pub perception: RawPerception,
    pub saves: RawSaves,
    pub traits: RawTraits,
    pub items: RawItems,
}

impl FoundryCreature {
    pub fn init_from_json(json: Value) -> Option<FoundryCreature> {
        let creature_type = json_utils::get_field_from_json(&json, "type")
            .as_str()
            .unwrap()
            .to_string();
        if !creature_type.eq_ignore_ascii_case("npc") {
            return None;
        }

        let system_json = json_utils::get_field_from_json(&json, "system");
        let abilities_json = json_utils::get_field_from_json(&system_json, "abilities");
        let attributes_json = json_utils::get_field_from_json(&system_json, "attributes");
        let details_json = json_utils::get_field_from_json(&system_json, "details");
        let initiative_json = json_utils::get_field_from_json(&system_json, "initiative");
        let perception_json = json_utils::get_field_from_json(&system_json, "perception");
        let saves_json = json_utils::get_field_from_json(&system_json, "saves");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        let items_json = json_utils::get_field_from_json(&json, "items");

        let name = json_utils::get_field_from_json(&json, "name")
            .as_str()
            .unwrap()
            .to_string();
        Some(FoundryCreature {
            name,
            creature_type,

            abilities: RawAbilities::init_from_json(abilities_json),
            attributes: RawAttributes::init_from_json(attributes_json),
            details: RawDetails::init_from_json(details_json),
            initiative_ability: json_utils::get_field_from_json(&initiative_json, "statistic")
                .as_str()
                .unwrap()
                .to_string(),
            perception: RawPerception::init_from_json(perception_json),
            saves: RawSaves::init_from_json(saves_json),
            traits: RawTraits::init_from_json(traits_json),
            items: RawItems::init_from_json(items_json),
        })
    }
}
