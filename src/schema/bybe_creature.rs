use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeWeapon};
use crate::schema::bybe_metadata_enum::{RarityEnum, SizeEnum};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::spellcasting_entry::SpellCastingEntry;
use crate::schema::source_schema::creature::resistance::Resistance;
use crate::schema::source_schema::creature::sense::Sense;
use crate::schema::source_schema::creature::source_creature::{
    SourceCreature, SourceCreatureParsingError,
};
use crate::schema::source_schema::rules::{Iwr, Rule};
use itertools::Itertools;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

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
    pub resistances: Vec<Resistance>,
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
    pub vision: bool,
    pub senses: Vec<Sense>,

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

    pub weapons: Vec<BybeWeapon>,
    pub armors: Vec<BybeArmor>,
    pub items: Vec<BybeItem>,
    pub actions: Vec<Action>,
    pub n_of_focus_points: i64,
    pub spellcasting: Vec<SpellCastingEntry>,
    pub spells: Vec<Spell>,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Error)]
pub enum BybeCreatureParsingError {
    #[error("Source item could not be parsed")]
    SourceCreatureError(#[from] SourceCreatureParsingError),
}

impl TryFrom<&Value> for BybeCreature {
    type Error = BybeCreatureParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(Self::from(SourceCreature::try_from(json)?))
    }
}

impl From<SourceCreature> for BybeCreature {
    fn from(source_cr: SourceCreature) -> Self {
        let spellcasting_entries = source_cr
            .items
            .spellcasting_entry
            .iter()
            .map(|sce| {
                let curr_sce_spells = source_cr
                    .items
                    .spell_list
                    .iter()
                    .filter(|spell| spell.location_id == sce.raw_foundry_id)
                    .cloned()
                    .collect();
                SpellCastingEntry::from((sce, &curr_sce_spells, source_cr.details.level))
            })
            .collect();
        let rules: Vec<Rule> = source_cr
            .items
            .action_list
            .iter()
            .flat_map(|x| x.rules.clone())
            .collect();
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
            hp: source_cr.attributes.hp_values.hp,
            hp_details: source_cr.attributes.hp_details,
            ac_details: source_cr.attributes.ac_details,
            speed: source_cr.attributes.speed,
            immunities: get_all_immunities(
                source_cr.attributes.immunities,
                &rules,
                &source_cr.traits.traits,
            ),
            resistances: get_resistances_adding_iwr_rules(source_cr.attributes.resistances, &rules),
            weaknesses: get_weaknesses_adding_iwr_rules(source_cr.attributes.weakness, &rules),
            languages_details: source_cr.details.languages_details,
            languages: source_cr.details.languages,
            level: source_cr.details.level,
            license: source_cr.details.publication_info.license,
            remaster: source_cr.details.publication_info.remastered,
            source: source_cr.details.publication_info.source,
            initiative_ability: source_cr.initiative_ability,
            perception_mod: source_cr.perception.perception_modifier,
            perception_details: source_cr.perception.perception_details,
            vision: source_cr.perception.vision,
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
            armors: source_cr.items.armor_list,
            weapons: source_cr.items.weapon_list,
            items: source_cr.items.item_list,
            actions: source_cr.items.action_list,
            spells: source_cr.items.spell_list,
            spellcasting: spellcasting_entries,
            skills: source_cr.items.skill_list,
            n_of_focus_points: source_cr.resource.n_of_focus_points,
        }
    }
}

fn get_all_immunities(immunities: Vec<String>, rules: &[Rule], traits: &[String]) -> Vec<String> {
    let mut result = immunities.clone();

    result.extend(get_implicit_immunities_from_traits(traits));
    get_immunities_adding_iwr_rules(result, rules)
}

fn get_resistances_adding_iwr_rules(
    resistances: Vec<Resistance>,
    rules: &[Rule],
) -> Vec<Resistance> {
    resistances
        .into_iter()
        .map(|res| {
            rules
                .iter()
                .find(|rule| rule.key == Iwr::Resistance && rule.name == res.name)
                .map_or(res, |rule| Resistance::try_from(rule).unwrap())
        })
        .collect()
}

fn get_weaknesses_adding_iwr_rules(
    weaknesses: HashMap<String, i64>,
    rules: &[Rule],
) -> HashMap<String, i64> {
    let mut wk = weaknesses.clone();
    rules
        .iter()
        .filter(|r| r.key == Iwr::Weakness)
        .for_each(|rule| {
            wk.entry(rule.name.clone())
                .and_modify(|v| *v = rule.value)
                .or_insert(rule.value);
        });
    wk
}

fn get_immunities_adding_iwr_rules(immunities: Vec<String>, rules: &[Rule]) -> Vec<String> {
    rules
        .iter()
        .filter(|x| x.key == Iwr::Immunity)
        .map(|x| x.name.clone())
        .chain(immunities)
        .unique()
        .collect()
}

fn get_implicit_immunities_from_traits(traits: &[String]) -> Vec<String> {
    //Check out foundry logic in actor/creature/helper.ts
    let mut result = vec![];
    /*
    "Constructs are often mindless; they're immune to bleed damage, death effects, disease, healing,
    nonlethal attacks, poison, vitality, void, and the doomed, drained, fatigued, paralyzed, sickened, and
    unconscious conditions; and they might have Hardness based on the materials used to construct their bodies."
    – GMC pg. 328
     */
    if traits.contains(&"construct".to_string()) && !traits.contains(&"eidolon".to_string()) {
        result.extend(vec![
            "bleed".to_string(),
            "death-effects".to_string(),
            "disease".to_string(),
            "doomed".to_string(),
            "drained".to_string(),
            "fatigued".to_string(),
            "healing".to_string(),
            "nonlethal-attacks".to_string(),
            "paralyzed".to_string(),
            "poison".to_string(),
            "sickened".to_string(),
            "spirit".to_string(),
            "unconscious".to_string(),
            "vitality".to_string(),
            "void".to_string(),
        ]);
    }
    // "They are immune to all mental effects." – GMC pg. 331
    if traits.contains(&"mindless".to_string()) && !traits.contains(&"mental".to_string()) {
        result.extend(vec!["mental".to_string()]);
    }
    // "Swarms are immune to the grappled [sic], prone, and restrained conditions." – GMC pg. 334
    if traits.contains(&"swarm".to_string()) {
        result.extend(vec![
            "grabbed".to_string(),
            "prone".to_string(),
            "restrained".to_string(),
        ]);
    }
    result
}
