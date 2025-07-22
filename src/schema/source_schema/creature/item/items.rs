use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeWeapon};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::{Spell, SpellParsingError};
use crate::schema::source_schema::creature::item::spellcasting_entry::RawSpellCastingEntry;
use log::debug;
use serde_json::Value;
use thiserror::Error;

pub struct ItemLinkedToCreature {
    pub spell_list: Vec<Spell>,
    pub weapon_list: Vec<BybeWeapon>,
    pub armor_list: Vec<BybeArmor>,
    pub item_list: Vec<BybeItem>,
    pub action_list: Vec<Action>,
    pub spellcasting_entry: Vec<RawSpellCastingEntry>,
    pub skill_list: Vec<Skill>,
}

#[derive(Debug, Error)]
pub enum CreatureItemParsingError {
    #[error("Json should be a vector")]
    JsonIsNotAVector,
    #[error("Could not parse item type")]
    MissingItemType,
    #[error("Could not parse the system field")]
    MissingSystemField,
    #[error("Spell could not be parsed")]
    SpellError(#[from] SpellParsingError),
}

impl TryFrom<&Value> for ItemLinkedToCreature {
    type Error = CreatureItemParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let json_vec = json
            .as_array()
            .ok_or(CreatureItemParsingError::JsonIsNotAVector)?;
        let mut spellcasting_entry = Vec::new();
        let mut spells = Vec::new();
        let mut weapons = Vec::new();
        let mut armors = Vec::new();
        let mut actions = Vec::new();
        let mut skills = Vec::new();
        let mut items = Vec::new();
        for el in json_vec {
            match el
                .get("type")
                .and_then(|x| x.as_str())
                .map(|x| x.to_ascii_lowercase())
                .ok_or(CreatureItemParsingError::MissingItemType)?
                .as_str()
            {
                "spellcastingentry" => match RawSpellCastingEntry::try_from(el) {
                    Ok(se) => spellcasting_entry.push(se),
                    Err(e) => debug!("{e}"),
                },
                "spell" => {
                    // if it has ritual it's a ritual, we should parse differently
                    if el
                        .get("system")
                        .map(|x| x.get("ritual").is_none())
                        .ok_or(CreatureItemParsingError::MissingSystemField)?
                    {
                        if let Ok(sp) = Spell::try_from(el) {
                            spells.push(sp);
                        }
                    }
                }
                "melee" | "weapon" => match BybeWeapon::try_from((el, true)) {
                    Ok(wp) => weapons.push(wp),
                    Err(e) => debug!("{e}"),
                },
                "armor" => match BybeArmor::try_from((el, true)) {
                    Ok(armor) => armors.push(armor),
                    Err(e) => debug!("{e}"),
                },
                "action" => match Action::try_from(el) {
                    Ok(action) => actions.push(action),
                    Err(e) => debug!("{e}"),
                },
                "lore" => match Skill::try_from(el) {
                    Ok(skill) => skills.push(skill),
                    Err(e) => debug!("{e}"),
                },
                // "real" items, like the one for the shop
                "consumable" | "equipment" => match BybeItem::try_from((el, true)) {
                    Ok(item) => items.push(item),
                    Err(e) => debug!("{e}"),
                },
                // there are other options
                _ => {
                    // do nothing
                }
            }
        }
        Ok(ItemLinkedToCreature {
            spellcasting_entry,
            spell_list: spells,
            weapon_list: weapons,
            armor_list: armors,
            item_list: items,
            action_list: actions,
            skill_list: skills,
        })
    }
}
