use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeShield, BybeWeapon, WeaponDamageData};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::spellcasting_entry::SpellCastingEntry;
use crate::schema::source_schema::creature::resistance::Resistance;
use crate::schema::source_schema::creature::sense::Sense;
use crate::utils::game_system_enum::GameSystem;
use anyhow::Result;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{QueryBuilder, query_file};
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::collections::HashMap;
use std::str::FromStr;

pub async fn connect(db_path: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(db_path)?.create_if_missing(true);
    Ok(SqlitePool::connect_with(options).await?)
}

pub async fn insert_item_to_db(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    item: &BybeItem,
    cr_id: Option<i64>,
) -> Result<i64> {
    let item_id = insert_item(conn, gs, item).await?;
    insert_traits(conn, gs, &item.traits).await?;
    insert_item_trait_association(conn, gs, &item.traits, item_id).await?;
    if let Some(creature_id) = cr_id {
        insert_item_creature_association(conn, gs, item_id, creature_id, item.quantity).await?;
    }
    Ok(item_id)
}

pub async fn insert_shield_to_db(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    shield: &BybeShield,
    cr_id: Option<i64>,
) -> Result<i64> {
    // Don't creature useless links between item & creature.
    // Since item is as generic as possible the specializations
    // (weapon, armor, etc) should have a separate association table
    let item_id = insert_item_to_db(conn, gs, &shield.item_core, None).await?;

    let shield_id = insert_shield(conn, gs, shield, item_id).await?;

    if let Some(creature_id) = cr_id {
        insert_shield_creature_association(
            conn,
            gs,
            shield_id,
            creature_id,
            shield.item_core.quantity,
        )
        .await?;
    }
    insert_shield_trait_association(conn, gs, &shield.item_core.traits, shield_id).await?;

    Ok(shield_id)
}

pub async fn insert_weapon_to_db(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    wp: &BybeWeapon,
    cr_id: Option<i64>,
) -> Result<i64> {
    // Don't creature useless links between item & creature.
    // Since item is as generic as possible the specializations
    // (weapon, armor, etc) should have a separate association table
    let item_id = insert_item_to_db(conn, gs, &wp.item_core, None).await?;

    let wp_id = insert_weapon(conn, gs, wp, item_id).await?;
    if let Some(creature_id) = cr_id {
        insert_weapon_creature_association(conn, gs, wp_id, creature_id, wp.item_core.quantity)
            .await?;
    }

    insert_weapon_damage(conn, gs, &wp.damage_data, wp_id).await?;

    insert_runes(conn, gs, &wp.property_runes).await?;
    insert_weapon_rune_association(conn, gs, &wp.property_runes, wp_id).await?;

    insert_weapon_trait_association(conn, gs, &wp.item_core.traits, wp_id).await?;
    Ok(wp_id)
}

pub async fn insert_armor_to_db(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    armor: &BybeArmor,
    cr_id: Option<i64>,
) -> Result<i64> {
    // Don't creature useless links between item & creature.
    // Since item is as generic as possible the specializations
    // (weapon, armor, etc) should have a separate association table
    let item_id = insert_item_to_db(conn, gs, &armor.item_core, None).await?;

    let arm_id = insert_armor(conn, gs, armor, item_id).await?;

    if let Some(creature_id) = cr_id {
        insert_armor_creature_association(conn, gs, arm_id, creature_id, armor.item_core.quantity)
            .await?;
    }

    insert_runes(conn, gs, &armor.property_runes).await?;
    insert_armor_rune_association(conn, gs, &armor.property_runes, arm_id).await?;

    insert_armor_trait_association(conn, gs, &armor.item_core.traits, arm_id).await?;
    Ok(arm_id)
}

pub async fn insert_creature_to_db(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    cr: BybeCreature,
) -> Result<bool> {
    let cr_id = insert_creature(conn, gs, &cr).await?;
    insert_traits(conn, gs, &cr.traits).await?;
    insert_cr_trait_association(conn, gs, &cr.traits, cr_id).await?;
    insert_language_and_association(conn, gs, &cr.languages, cr_id).await?;
    insert_immunity_and_association(conn, gs, &cr.immunities, cr_id).await?;
    insert_sense_and_association(conn, gs, &cr.senses, cr_id).await?;
    insert_speeds(conn, gs, &cr.speed, cr_id).await?;
    insert_weaknesses(conn, gs, &cr.weaknesses, cr_id).await?;
    insert_resistances(conn, gs, &cr.resistances, cr_id).await?;
    for el in &cr.weapons {
        insert_weapon_to_db(conn, gs, el, Some(cr_id)).await?;
    }
    for el in &cr.armors {
        insert_armor_to_db(conn, gs, el, Some(cr_id)).await?;
    }
    for el in &cr.spellcasting {
        let sc_entry_id = insert_spellcasting_entry(conn, gs, el, cr_id).await?;
        for (slot, spells) in el.spell_slots.clone() {
            for spell in spells {
                let spell_id = insert_spell(conn, gs, &spell, slot, cr_id, sc_entry_id).await?;
                insert_traits(conn, gs, &spell.traits.traits).await?;
                insert_spell_trait_association(conn, gs, &spell.traits.traits, spell_id).await?;
                insert_tradition_and_association(conn, gs, &spell.traits.traditions, spell_id)
                    .await?;
            }
        }
    }
    for el in &cr.actions {
        let action_id = insert_action(conn, gs, el, cr_id).await?;
        insert_traits(conn, gs, &el.traits.traits).await?;
        insert_action_trait_association(conn, gs, &el.traits.traits, action_id).await?;
    }
    for el in &cr.skills {
        let skill_id = insert_skill(conn, gs, el, cr_id).await?;
        insert_skill_modifier_variant_table(conn, gs, &el.variant_label, cr_id, skill_id).await?;
    }
    for el in &cr.items {
        insert_item_to_db(conn, gs, el, Some(cr_id)).await?;
    }
    Ok(true)
}

pub async fn update_with_aon_data(conn: &mut Transaction<'_, Sqlite>) -> Result<bool> {
    query_file!("src/db/raw_queries/update_mon_w_aon_data.sql")
        .execute(&mut **conn)
        .await?;
    query_file!("src/db/raw_queries/update_npc_w_aon_data.sql")
        .execute(&mut **conn)
        .await?;
    Ok(true)
}

pub async fn insert_scales_values_to_db(conn: &mut Transaction<'_, Sqlite>) -> Result<bool> {
    query_file!("src/db/raw_queries/scales.sql")
        .execute(&mut **conn)
        .await?;
    Ok(true)
}

async fn insert_traits(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!("INSERT OR IGNORE INTO {gs}_trait_table (name) "))
            .push_values(traits, |mut b, el| {
                b.push_bind(el);
            })
            .build()
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_weapon_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_weapon_association_table (weapon_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_shield_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_shield_association_table (shield_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_armor_association_table (armor_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_item_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_item_association_table (item_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_cr_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_creature_association_table (creature_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_language_and_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    languages: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in languages {
        sqlx::query(
            format!("INSERT OR IGNORE INTO {gs}_language_table (name) VALUES (?)",).as_str(),
        )
        .bind(el)
        .execute(&mut **conn)
        .await?;
        sqlx::query(
            format!(
                "INSERT OR IGNORE INTO {gs}_language_creature_association_table (creature_id, language_id) VALUES (?, ?)"
            )
            .as_str(),
        )
        .bind(id)
        .bind(el)
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_immunity_and_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    immunities: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in immunities {
        sqlx::query(
            format!("INSERT OR IGNORE INTO {gs}_immunity_table (name) VALUES (?)").as_str(),
        )
        .bind(el)
        .execute(&mut **conn)
        .await?;
        sqlx::query(
            format!(
                "INSERT OR IGNORE INTO {gs}_immunity_creature_association_table (creature_id, immunity_id) VALUES (?, ?)"
            )
            .as_str(),
        )
        .bind(id)
        .bind(el)
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_sense_and_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    senses: &Vec<Sense>,
    id: i64,
) -> Result<bool> {
    for el in senses {
        let sense_id = sqlx::query(format!(
            "INSERT OR IGNORE INTO {gs}_sense_table (id, name, range, acuity) VALUES (?, ?, ?, ?)",).as_str())
        .bind(None::<i64>)
        .bind(&el.name)
        .bind(el.range)
        .bind(&el.acuity)
        .execute(&mut **conn)
        .await?
        .last_insert_rowid();
        sqlx::query(
            format!(
                "INSERT OR IGNORE INTO {gs}_sense_creature_association_table (creature_id, sense_id) VALUES (?, ?)"
            ).as_str()
        )
        .bind(id)
        .bind(sense_id)
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_speeds(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    speed: &HashMap<String, i64>,
    id: i64,
) -> Result<bool> {
    QueryBuilder::new(format!(
        "INSERT INTO {gs}_speed_table (creature_id, name, value) "
    ))
    .push_values(speed, |mut b, (speed_type, speed_value)| {
        b.push_bind(id).push_bind(speed_type).push_bind(speed_value);
    })
    .build()
    .execute(&mut **conn)
    .await?;

    Ok(true)
}

async fn insert_resistances(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    resistances: &Vec<Resistance>,
    id: i64,
) -> Result<bool> {
    for res in resistances {
        let res_id = sqlx::query(
            format!("INSERT OR IGNORE INTO {gs}_resistance_table (id, creature_id, name, value) VALUES (?, ?, ?, ?)").as_str())
        .bind(None::<i64>)
        .bind(id)
        .bind(&res.name)
        .bind(res.value)
        .execute(&mut **conn)
        .await?
        .last_insert_rowid();

        insert_resistance_double_vs(conn, gs, res_id, res.double_vs.clone()).await?;
        insert_resistance_exception_vs(conn, gs, res_id, res.exceptions.clone()).await?;
    }

    Ok(true)
}

async fn insert_resistance_double_vs(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    res_id: i64,
    double_vs: Vec<String>,
) -> Result<bool> {
    if !double_vs.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_resistance_double_vs_table (resistance_id, vs_name) "
        ))
        .push_values(double_vs, |mut b, el| {
            b.push_bind(res_id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }

    Ok(true)
}

async fn insert_resistance_exception_vs(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    res_id: i64,
    exception_vs: Vec<String>,
) -> Result<bool> {
    if !exception_vs.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_resistance_exception_vs_table (resistance_id, vs_name) "
        ))
        .push_values(exception_vs, |mut b, el| {
            b.push_bind(res_id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_weaknesses(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    weaknesses: &HashMap<String, i64>,
    id: i64,
) -> Result<bool> {
    if !weaknesses.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_weakness_table (creature_id, name, value) "
        ))
        .push_values(weaknesses, |mut b, (weak_type, weak_value)| {
            b.push_bind(id).push_bind(weak_type).push_bind(weak_value);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }

    Ok(true)
}

async fn insert_item(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    item: &BybeItem,
) -> Result<i64> {
    let size = item.size.to_string();
    let rarity = item.rarity.to_string();
    // we check if a similar base item is already present
    // if it is then we return the id without inserting a new entry
    match sqlx::query(
        format!(
            "INSERT INTO {gs}_item_table VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
            ?, ?
        );
    "
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(&item.name)
    .bind(item.bulk)
    .bind(&item.base_item)
    .bind(&item.category)
    .bind(&item.description)
    .bind(item.hardness)
    .bind(item.hp)
    .bind(item.level)
    .bind(item.price)
    .bind(&item.usage)
    .bind(&item.group)
    .bind(&item.item_type)
    .bind(item.is_derived)
    .bind(&item.material_grade)
    .bind(&item.material_type)
    .bind(item.number_of_uses)
    .bind(&item.license)
    .bind(item.remaster)
    .bind(&item.source)
    .bind(&rarity)
    .bind(&size)
    .execute(&mut **conn)
    .await
    {
        Ok(r) => Ok(r.last_insert_rowid()),
        Err(_) => {
            let x: Option<i64> = sqlx::query_scalar(
                format!(
                    "SELECT id FROM {gs}_item_table WHERE
                    name = ? AND bulk = ? AND description = ? AND hardness = ? AND
                    hp = ? AND level = ? AND price = ? AND item_type = ? AND license = ? AND
                    remaster = ? AND source = ? AND rarity = ? AND size = ?
                "
                )
                .as_str(),
            )
            .bind(&item.name)
            .bind(item.bulk)
            .bind(&item.description)
            .bind(item.hardness)
            .bind(item.hp)
            .bind(item.level)
            .bind(item.price)
            .bind(&item.item_type)
            .bind(&item.license)
            .bind(item.remaster)
            .bind(&item.source)
            .bind(&rarity)
            .bind(&size)
            .fetch_optional(&mut **conn)
            .await?;
            match x {
                Some(i) => Ok(i),
                None => anyhow::bail!("Could not fetch id"),
            }
        }
    }
}

async fn insert_item_creature_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    item_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(
        format!(
            "INSERT OR IGNORE INTO {gs}_item_creature_association_table
            (item_id, creature_id, quantity) VALUES (?, ?, ?)",
        )
        .as_str(),
    )
    .bind(item_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn insert_creature(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    cr: &BybeCreature,
) -> Result<i64> {
    let size = cr.size.to_string();
    let rarity = cr.rarity.to_string();
    Ok(sqlx::query(
        format!(
            "
            INSERT INTO {gs}_creature_table VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?
            )"
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(&cr.name)
    .bind(None::<i64>) //aon_id, need to fetch it manually
    .bind(cr.charisma)
    .bind(cr.constitution)
    .bind(cr.dexterity)
    .bind(cr.intelligence)
    .bind(cr.strength)
    .bind(cr.wisdom)
    .bind(cr.ac)
    .bind(cr.hp)
    .bind(&cr.hp_details)
    .bind(&cr.ac_details)
    .bind(&cr.languages_details)
    .bind(cr.level)
    .bind(&cr.license)
    .bind(cr.remaster)
    .bind(&cr.source)
    .bind(&cr.initiative_ability)
    .bind(cr.perception_mod)
    .bind(&cr.perception_details)
    .bind(cr.vision)
    .bind(cr.fortitude_mod)
    .bind(cr.reflex_mod)
    .bind(cr.will_mod)
    .bind(&cr.fortitude_detail)
    .bind(&cr.reflex_detail)
    .bind(&cr.will_detail)
    .bind(rarity)
    .bind(size)
    .bind(None::<String>) // type, source says NPC always..
    .bind(None::<String>) // family, source does not have it
    .bind(cr.n_of_focus_points)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

// CREATURE CORE DONE
// SHIELD CORE START

async fn insert_shield(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    shield: &BybeShield,
    item_id: i64,
) -> Result<i64> {
    Ok(
        sqlx::query(format!("INSERT INTO {gs}_shield_table VALUES (?, ?, ?, ?, ?)").as_str())
            .bind(None::<i64>) // id, autoincrement
            .bind(shield.ac_bonus)
            .bind(shield.n_of_reinforcing_runes)
            .bind(shield.speed_penalty)
            .bind(item_id)
            .execute(&mut **conn)
            .await?
            .last_insert_rowid(),
    )
}

async fn insert_shield_creature_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    shield_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(
        format!(
            "INSERT OR IGNORE INTO {gs}_shield_creature_association_table
            (shield_id, creature_id, quantity) VALUES (?, ?, ?)"
        )
        .as_str(),
    )
    .bind(shield_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

// WEAPON CORE START
async fn insert_weapon(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    wp: &BybeWeapon,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query(
        format!("INSERT INTO {gs}_weapon_table VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)").as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(wp.to_hit_bonus)
    .bind(wp.splash_dmg)
    .bind(wp.n_of_potency_runes)
    .bind(wp.n_of_striking_runes)
    .bind(wp.range)
    .bind(&wp.reload)
    .bind(&wp.weapon_type)
    .bind(item_id)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_weapon_damage(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    dmg_data: &Vec<WeaponDamageData>,
    wp_id: i64,
) -> Result<()> {
    for el in dmg_data {
        sqlx::query(
            format!("INSERT INTO {gs}_weapon_damage_table VALUES (?, ?, ?, ?, ?, ?)").as_str(),
        )
        .bind(None::<i64>) // id, autoincrement
        .bind(el.bonus_dmg)
        .bind(&el.dmg_type)
        .bind(el.n_of_dice)
        .bind(el.die_size)
        .bind(wp_id)
        .execute(&mut **conn)
        .await?;
    }
    Ok(())
}

async fn insert_weapon_creature_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    weapon_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(
        format!(
            "INSERT OR IGNORE INTO {gs}_weapon_creature_association_table
            (weapon_id, creature_id, quantity) VALUES (?, ?, ?)"
        )
        .as_str(),
    )
    .bind(weapon_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

// WEAPON CORE END

// ARMOR CORE START

async fn insert_armor(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    armor: &BybeArmor,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query(
        format!(
            "
            INSERT INTO {gs}_armor_table VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?
            )"
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(armor.ac_bonus)
    .bind(armor.check_penalty)
    .bind(armor.dex_cap)
    .bind(armor.n_of_potency_runes)
    .bind(armor.n_of_resilient_runes)
    .bind(armor.speed_penalty)
    .bind(armor.strength_required)
    .bind(item_id)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}
async fn insert_armor_creature_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    armor_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(
        format!(
            "INSERT OR IGNORE INTO {gs}_armor_creature_association_table
            (armor_id, creature_id, quantity) VALUES (?, ?, ?)"
        )
        .as_str(),
    )
    .bind(armor_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

// ARMOR CORE END
// SPELL CORE START

async fn insert_spellcasting_entry(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    spellcasting_entry: &SpellCastingEntry,
    id: i64,
) -> Result<i64> {
    Ok(sqlx::query(
        format!(
            "INSERT INTO {gs}_spellcasting_entry_table VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?
        )"
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(&spellcasting_entry.name)
    .bind(spellcasting_entry.is_flexible)
    .bind(&spellcasting_entry.type_of_spellcaster)
    .bind(spellcasting_entry.dc_modifier)
    .bind(spellcasting_entry.atk_modifier)
    .bind(&spellcasting_entry.tradition)
    .bind(spellcasting_entry.heighten_level)
    .bind(id)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_spell_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_spell_association_table (spell_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_tradition_and_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    tradition: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !tradition.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_tradition_table (name) "
        ))
        .push_values(tradition, |mut b, el| {
            b.push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;

        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_tradition_spell_association_table (spell_id, tradition_id) "
        ))
        .push_values(tradition, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_spell(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    spell: &Spell,
    slot: i64,
    cr_id: i64,
    spellcasting_entry_id: i64,
) -> Result<i64> {
    let (area_type, area_value) = match spell.area.clone() {
        Some(data) => (Some(data.area_type), Some(data.area_value)),
        None => (None, None),
    };
    let (save_throw, save_throw_mod) = match spell.saving_throw.clone() {
        Some(data) => (Some(data.statistic), Some(data.basic)),
        None => (None, None),
    };
    Ok(sqlx::query(
        format!(
            "
            INSERT INTO {gs}_spell_table VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )"
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(&spell.name)
    .bind(area_type)
    .bind(area_value)
    .bind(spell.counteraction)
    .bind(save_throw)
    .bind(save_throw_mod)
    .bind(spell.sustained)
    .bind(&spell.duration)
    .bind(spell.level)
    .bind(&spell.range)
    .bind(&spell.target)
    .bind(&spell.actions)
    .bind(&spell.publication_info.license)
    .bind(spell.publication_info.remastered)
    .bind(&spell.publication_info.source)
    .bind(&spell.traits.rarity)
    .bind(slot)
    .bind(cr_id)
    .bind(spellcasting_entry_id)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

// ACTION

async fn insert_action(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    action: &Action,
    cr_id: i64,
) -> Result<i64> {
    Ok(sqlx::query(
        format!(
            "
            INSERT INTO {gs}_action_table VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )"
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(&action.name)
    .bind(&action.action_type)
    .bind(action.n_of_actions)
    .bind(&action.category)
    .bind(&action.description)
    .bind(&action.publication_info.license)
    .bind(action.publication_info.remastered)
    .bind(&action.publication_info.source)
    .bind(&action.slug)
    .bind(&action.traits.rarity)
    .bind(cr_id)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_action_trait_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_trait_action_association_table (action_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_skill(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    skill: &Skill,
    cr_id: i64,
) -> Result<i64> {
    Ok(sqlx::query(
        format!(
            "
            INSERT INTO {gs}_skill_table VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?
            )",
        )
        .as_str(),
    )
    .bind(None::<i64>) // id, autoincrement
    .bind(&skill.name)
    .bind(&skill.description)
    .bind(skill.modifier)
    .bind(skill.proficiency)
    .bind(&skill.publication_info.license)
    .bind(skill.publication_info.remastered)
    .bind(&skill.publication_info.source)
    .bind(cr_id)
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_skill_modifier_variant_table(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    skill_labels: &Vec<String>,
    cr_id: i64,
    skill_id: i64,
) -> Result<bool> {
    if !skill_labels.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_creature_skill_label_table (creature_id, skill_id, skill_label) "
        ))
        .push_values(skill_labels, |mut b, el| {
            b.push_bind(cr_id).push_bind(skill_id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_runes(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    runes: &Vec<String>,
) -> Result<bool> {
    if !runes.is_empty() {
        QueryBuilder::new(format!("INSERT OR IGNORE INTO {gs}_rune_table (name) "))
            .push_values(runes, |mut b, el| {
                b.push_bind(el);
            })
            .build()
            .execute(&mut **conn)
            .await?;
    }

    Ok(true)
}

async fn insert_weapon_rune_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    runes: &Vec<String>,
    wp_id: i64,
) -> Result<bool> {
    if !runes.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_rune_weapon_association_table (weapon_id, rune_id) "
        ))
        .push_values(runes, |mut b, el| {
            b.push_bind(wp_id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor_rune_association(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    runes: &Vec<String>,
    arm_id: i64,
) -> Result<bool> {
    if !runes.is_empty() {
        QueryBuilder::new(format!(
            "INSERT OR IGNORE INTO {gs}_rune_armor_association_table (armor_id, rune_id) "
        ))
        .push_values(runes, |mut b, el| {
            b.push_bind(arm_id).push_bind(el);
        })
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}
