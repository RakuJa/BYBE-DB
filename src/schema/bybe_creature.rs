use crate::schema::bybe_creature_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::foundry_schema::creature::foundry_creature::FoundryCreature;
use crate::schema::foundry_schema::creature::item::spell::Spell;
use crate::schema::foundry_schema::creature::item::weapon::Weapon;
use std::collections::HashMap;

pub struct BybeCreature {
    pub name: String,
    pub creature_type: Option<String>,

    // abilities mod
    pub charisma: i64,
    pub constitution: i64,
    pub dexterity: i64,
    pub intelligence: i64,
    pub strength: i64,
    pub wisdom: i64,

    // attributes
    pub ac: i64,
    pub hp: i64,
    pub hp_details: String,
    pub ac_details: String,
    pub speed: HashMap<String, i64>,
    pub immunities: Vec<String>,
    pub resistances: HashMap<String, i64>,
    pub weaknesses: HashMap<String, i64>,

    // details
    pub languages_details: String,
    pub languages: Vec<String>,
    pub level: i64,
    // source details (title, license, remastered)
    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub initiative_ability: String,

    // Awareness/eyes
    pub perception_mod: i64,
    pub perception_details: String,
    pub senses: Vec<String>,

    // Saves
    pub fortitude_mod: i64,
    pub reflex_mod: i64,
    pub will_mod: i64,

    pub fortitude_detail: String,
    pub reflex_detail: String,
    pub will_detail: String,

    pub rarity: RarityEnum,
    pub size: SizeEnum,
    pub traits: Vec<String>,

    pub weapons: Vec<Weapon>,
    pub spell_casting: Option<String>,
    pub spells: Vec<Spell>,
}

impl BybeCreature {
    pub fn init_from_foundry_creature(foundry_cr: FoundryCreature) -> BybeCreature {
        BybeCreature {
            name: foundry_cr.name,
            creature_type: None,
            charisma: foundry_cr.abilities.charisma,
            constitution: foundry_cr.abilities.constitution,
            dexterity: foundry_cr.abilities.dexterity,
            intelligence: foundry_cr.abilities.intelligence,
            strength: foundry_cr.abilities.strength,
            wisdom: foundry_cr.abilities.wisdom,
            ac: foundry_cr.attributes.ac,
            hp: foundry_cr.attributes.hp,
            hp_details: foundry_cr.attributes.hp_details,
            ac_details: foundry_cr.attributes.ac_details,
            speed: foundry_cr.attributes.speed,
            immunities: foundry_cr.attributes.immunities,
            resistances: foundry_cr.attributes.resistances,
            weaknesses: foundry_cr.attributes.weakness,
            languages_details: foundry_cr.details.languages_details,
            languages: foundry_cr.details.languages,
            level: foundry_cr.details.level,
            license: foundry_cr.details.publication_info.license,
            remaster: foundry_cr.details.publication_info.remastered,
            source: foundry_cr.details.publication_info.source,
            initiative_ability: foundry_cr.initiative_ability,
            perception_mod: foundry_cr.perception.perception_modifier,
            perception_details: foundry_cr.perception.perception_details,
            senses: foundry_cr.perception.senses,
            fortitude_mod: foundry_cr.saves.fortitude,
            reflex_mod: foundry_cr.saves.reflex,
            will_mod: foundry_cr.saves.will,
            fortitude_detail: foundry_cr.saves.fortitude_detail,
            reflex_detail: foundry_cr.saves.reflex_detail,
            will_detail: foundry_cr.saves.will_detail,
            rarity: foundry_cr
                .traits
                .rarity
                .as_str()
                .parse()
                .unwrap_or(RarityEnum::Common),
            size: foundry_cr
                .traits
                .size
                .as_str()
                .parse()
                .unwrap_or(SizeEnum::Medium),
            traits: foundry_cr.traits.traits,
            weapons: foundry_cr.items.weapon_list,
            spells: foundry_cr.items.spell_list,
            spell_casting: foundry_cr.items.spell_casting,
        }
    }
}
