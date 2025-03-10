use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeWeapon};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::spellcasting_entry::RawSpellCastingEntry;
use serde_json::Value;

pub struct ItemLinkedToCreature {
    pub spell_list: Vec<Spell>,
    pub weapon_list: Vec<BybeWeapon>,
    pub armor_list: Vec<BybeArmor>,
    pub item_list: Vec<BybeItem>,
    pub action_list: Vec<Action>,
    pub spellcasting_entry: Vec<RawSpellCastingEntry>,
    pub skill_list: Vec<Skill>,
}

impl ItemLinkedToCreature {
    pub fn init_from_json(json: Value) -> ItemLinkedToCreature {
        let json_vec = json
            .as_array()
            .expect("Items entry is not formatted as a vector, Abort.");
        let mut spellcasting_entry = Vec::new();
        let mut spells = Vec::new();
        let mut weapons = Vec::new();
        let mut armors = Vec::new();
        let mut actions = Vec::new();
        let mut skills = Vec::new();
        let mut items = Vec::new();
        for el in json_vec {
            let curr_el_type = el
                .get("type")
                .expect("Field type in item not formatted as expected");
            let curr_type = curr_el_type.as_str().unwrap().to_string();
            match curr_type.to_ascii_lowercase().as_str() {
                "spellcastingentry" => {
                    spellcasting_entry.push(RawSpellCastingEntry::init_from_json(el));
                }
                "spell" => {
                    // if it has ritual it's a ritual, we should parse differently
                    if el.get("system").unwrap().get("ritual").is_none() {
                        spells.push(Spell::init_from_json(el));
                    }
                }
                "melee" | "weapon" => {
                    if let Some(wp) = BybeWeapon::init_from_json(el) {
                        weapons.push(wp);
                    }
                }
                "armor" => {
                    if let Some(armor) = BybeArmor::init_from_json(el) {
                        armors.push(armor)
                    }
                }
                "action" => {
                    actions.push(Action::init_from_json(el));
                }
                "lore" => {
                    skills.push(Skill::init_from_json(el));
                }
                // "real" items, like the one for the shop
                "consumable" | "equipment" => {
                    if let Some(item) = BybeItem::init_from_json(el) {
                        items.push(item)
                    }
                }
                // there are other options
                _ => {
                    // do nothing
                }
            }
        }
        ItemLinkedToCreature {
            spellcasting_entry,
            spell_list: spells,
            weapon_list: weapons,
            armor_list: armors,
            item_list: items,
            action_list: actions,
            skill_list: skills,
        }
    }
}
