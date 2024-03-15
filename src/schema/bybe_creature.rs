use crate::schema::bybe_creature_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::spell_casting_entry::SpellCastingEntry;
use crate::schema::source_schema::creature::item::weapon::Weapon;
use crate::schema::source_schema::creature::source_creature::SourceCreature;
use std::collections::HashMap;

#[derive(Clone)]
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
    pub actions: Vec<Action>,
    pub spell_casting: Option<SpellCastingEntry>,
    pub spells: Vec<Spell>,
    pub skills: Vec<Skill>,
}

impl BybeCreature {
    pub fn init_from_source_creature(source_cr: SourceCreature) -> BybeCreature {
        BybeCreature {
            name: source_cr.name,
            creature_type: None,
            charisma: source_cr.abilities.charisma,
            constitution: source_cr.abilities.constitution,
            dexterity: source_cr.abilities.dexterity,
            intelligence: source_cr.abilities.intelligence,
            strength: source_cr.abilities.strength,
            wisdom: source_cr.abilities.wisdom,
            ac: source_cr.attributes.ac,
            hp: source_cr.attributes.hp,
            hp_details: source_cr.attributes.hp_details,
            ac_details: source_cr.attributes.ac_details,
            speed: source_cr.attributes.speed,
            immunities: source_cr.attributes.immunities,
            resistances: source_cr.attributes.resistances,
            weaknesses: source_cr.attributes.weakness,
            languages_details: source_cr.details.languages_details,
            languages: source_cr.details.languages,
            level: source_cr.details.level,
            license: source_cr.details.publication_info.license,
            remaster: source_cr.details.publication_info.remastered,
            source: source_cr.details.publication_info.source,
            initiative_ability: source_cr.initiative_ability,
            perception_mod: source_cr.perception.perception_modifier,
            perception_details: source_cr.perception.perception_details,
            senses: source_cr.perception.senses,
            fortitude_mod: source_cr.saves.fortitude,
            reflex_mod: source_cr.saves.reflex,
            will_mod: source_cr.saves.will,
            fortitude_detail: source_cr.saves.fortitude_detail,
            reflex_detail: source_cr.saves.reflex_detail,
            will_detail: source_cr.saves.will_detail,
            rarity: source_cr
                .traits
                .rarity
                .as_str()
                .parse()
                .unwrap_or(RarityEnum::Common),
            size: source_cr
                .traits
                .size
                .as_str()
                .parse()
                .unwrap_or(SizeEnum::Medium),
            traits: source_cr.traits.traits,
            weapons: source_cr.items.weapon_list,
            actions: source_cr.items.action_list,
            spells: source_cr.items.spell_list,
            spell_casting: source_cr.items.spell_casting_entry,
            skills: source_cr.items.skill_list,
        }
    }
}
