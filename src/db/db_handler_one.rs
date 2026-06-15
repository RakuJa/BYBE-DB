use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_hazard::BybeHazard;
use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeShield, BybeWeapon, WeaponDamageData};
use crate::schema::bybe_trait::Trait;
use crate::schema::source_schema::common::range_data::RangeData;
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::spellcasting_entry::SpellCastingEntry;
use crate::schema::source_schema::creature::resistance::Resistance;
use crate::schema::source_schema::creature::sense::Sense;
use crate::utils::game_system_enum::GameSystem;
use anyhow::Result;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use std::collections::HashMap;

pub async fn connect(db_url: &str) -> Result<PgPool> {
    Ok(PgPool::connect(db_url).await?)
}

pub async fn drop_tables_except(conn: &PgPool, exclude: &[&str]) -> Result<(), sqlx::Error> {
    let mut exclusions = vec!["_sqlx_migrations"];
    exclusions.extend_from_slice(exclude);

    let tables: Vec<(String,)> =
        sqlx::query_as("SELECT tablename FROM pg_tables WHERE schemaname = 'public'")
            .fetch_all(conn)
            .await?;

    for (table_name,) in tables {
        if !exclusions.contains(&table_name.as_str()) {
            sqlx::query(sqlx::AssertSqlSafe(format!("DELETE FROM {}", table_name)))
                .execute(conn)
                .await?;
        }
    }

    Ok(())
}

pub async fn insert_hazard_to_db(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    hz: &BybeHazard,
) -> Result<bool> {
    let hz_id = insert_hazard(conn, gs, hz).await.unwrap();
    insert_traits(conn, gs, &hz.traits).await.unwrap();
    insert_hazard_trait_association(conn, gs, &hz.traits, hz_id)
        .await
        .unwrap();

    for el in &hz.actions {
        let action_id = insert_action(conn, gs, el).await.unwrap();
        insert_traits(conn, gs, &el.traits.traits).await.unwrap();
        insert_action_trait_association(conn, gs, &el.traits.traits, action_id)
            .await
            .unwrap();
        insert_action_hazard_association(conn, gs, action_id, hz_id)
            .await
            .unwrap();
    }
    Ok(true)
}

async fn insert_hazard_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_hazard_association_table (hazard_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_action_hazard_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    action_id: i64,
    hz_id: i64,
) -> Result<bool> {
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_action_hazard_association_table (action_id, hazard_id) VALUES ($1, $2)"
    )))
    .bind(action_id)
    .bind(hz_id)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn insert_hazard(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    hz: &BybeHazard,
) -> Result<i64> {
    let size = hz.size.to_string();
    let rarity = hz.rarity.to_string();
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_hazard_table (
            foundry_id, name, ac, hardness, has_health, hp,
            stealth, stealth_detail, description, disable_description,
            reset_description, routine_description, is_complex, level,
            license, remaster, source,
            fortitude, reflex, will,
            fortitude_detail, reflex_detail, will_detail,
            rarity, size
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10,
            $11, $12, $13, $14,
            $15, $16, $17,
            $18, $19, $20,
            $21, $22, $23,
            $24, $25
        ) ON CONFLICT (foundry_id) DO UPDATE SET
            name = excluded.name,
            ac = excluded.ac,
            hardness = excluded.hardness,
            has_health = excluded.has_health,
            hp = excluded.hp,
            stealth = excluded.stealth,
            stealth_detail = excluded.stealth_detail,
            description = excluded.description,
            disable_description = excluded.disable_description,
            reset_description = excluded.reset_description,
            routine_description = excluded.routine_description,
            is_complex = excluded.is_complex,
            level = excluded.level,
            license = excluded.license,
            remaster = excluded.remaster,
            source = excluded.source,
            fortitude = excluded.fortitude,
            reflex = excluded.reflex,
            will = excluded.will,
            fortitude_detail = excluded.fortitude_detail,
            reflex_detail = excluded.reflex_detail,
            will_detail = excluded.will_detail,
            rarity = excluded.rarity,
            size = excluded.size"
    )))
    .bind(&hz.foundry_id)
    .bind(&hz.name)
    .bind(hz.ac_value)
    .bind(hz.hardness)
    .bind(hz.has_health)
    .bind(hz.hp_values.hp)
    .bind(hz.stealth)
    .bind(hz.stealth_detail.to_string())
    .bind(hz.description.to_string())
    .bind(hz.disable_description.to_string())
    .bind(hz.reset_description.to_string())
    .bind(hz.routine_description.to_string())
    .bind(hz.is_complex)
    .bind(hz.level)
    .bind(hz.publication_info.license.clone())
    .bind(hz.publication_info.remastered)
    .bind(hz.publication_info.source.clone())
    .bind(hz.saves.fortitude)
    .bind(hz.saves.reflex)
    .bind(hz.saves.will)
    .bind(hz.saves.fortitude_detail.clone())
    .bind(hz.saves.reflex_detail.clone())
    .bind(hz.saves.will_detail.clone())
    .bind(rarity)
    .bind(size)
    .execute(&mut **conn)
    .await?;
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT id FROM {gs}_hazard_table WHERE foundry_id = $1"
    )))
    .bind(&hz.foundry_id)
    .fetch_one(&mut **conn)
    .await?)
}

pub async fn insert_item_to_db(
    conn: &mut Transaction<'_, Postgres>,
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
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    shield: &BybeShield,
    cr_id: Option<i64>,
) -> Result<i64> {
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
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    wp: &BybeWeapon,
    cr_id: Option<i64>,
) -> Result<i64> {
    let item_id = insert_item_to_db(conn, gs, &wp.item_core, None).await?;

    let wp_id = insert_weapon(conn, gs, wp, item_id).await?;
    if let Some(range) = wp.range.clone() {
        let range_id = insert_range(conn, gs, range).await?;
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_weapon_range_association_table (range_id, weapon_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )))
        .bind(range_id)
        .bind(wp_id)
        .execute(&mut **conn)
        .await?;
    }
    if let Some(creature_id) = cr_id {
        insert_weapon_creature_association(conn, gs, wp_id, creature_id, wp.item_core.quantity)
            .await?;
    }
    insert_weapon_damage(conn, gs, &wp.damage_data, wp_id).await?;

    insert_runes(conn, gs, &wp.property_runes).await?;
    insert_weapon_rune_association(conn, gs, &wp.property_runes, wp_id).await?;

    insert_weapon_trait_association(conn, gs, &wp.item_core.traits, wp_id).await?;
    insert_weapon_attack_effect_association(conn, gs, &wp.attack_effects, wp_id).await?;
    Ok(wp_id)
}

pub async fn insert_armor_to_db(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    armor: &BybeArmor,
    cr_id: Option<i64>,
) -> Result<i64> {
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
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    cr: &BybeCreature,
) -> Result<bool> {
    let cr_id = insert_creature(conn, gs, cr).await?;
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
                if let Some(range) = spell.range {
                    let range_id = insert_range(conn, gs, range).await?;
                    sqlx::query(sqlx::AssertSqlSafe(format!(
                        "INSERT INTO {gs}_spell_range_association_table (range_id, spell_id) \
                    VALUES ($1, $2) ON CONFLICT DO NOTHING"
                    )))
                    .bind(range_id)
                    .bind(spell_id)
                    .execute(&mut **conn)
                    .await?;
                }
                insert_traits(conn, gs, &spell.traits.traits).await?;
                insert_spell_trait_association(conn, gs, &spell.traits.traits, spell_id).await?;
                insert_tradition_and_association(conn, gs, &spell.traits.traditions, spell_id)
                    .await?;
            }
        }
    }
    for el in &cr.actions {
        let action_id = insert_action(conn, gs, el).await?;
        insert_traits(conn, gs, &el.traits.traits).await?;
        insert_action_trait_association(conn, gs, &el.traits.traits, action_id).await?;
        insert_action_creature_association(conn, gs, action_id, cr_id).await?;
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

async fn insert_action_creature_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    action_id: i64,
    cr_id: i64,
) -> Result<bool> {
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_creature_action_association_table (action_id, creature_id) VALUES ($1, $2)"
    )))
    .bind(action_id)
    .bind(cr_id)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

pub async fn update_with_aon_data(conn: &mut Transaction<'_, Postgres>) -> Result<bool> {
    sqlx::raw_sql(include_str!("raw_queries/update_mon_w_aon_data.sql"))
        .execute(&mut **conn)
        .await?;
    sqlx::raw_sql(include_str!("raw_queries/update_npc_w_aon_data.sql"))
        .execute(&mut **conn)
        .await?;
    Ok(true)
}

pub async fn insert_scales_values_to_db(conn: &mut Transaction<'_, Postgres>) -> Result<bool> {
    sqlx::raw_sql(include_str!("raw_queries/scales.sql"))
        .execute(&mut **conn)
        .await?;
    Ok(true)
}

async fn insert_traits(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &[String],
) -> Result<bool> {
    if !traits.is_empty() {
        let complete_traits = traits
            .iter()
            .map(|x| Trait::builder().name(x).game_system(gs).build())
            .collect::<Vec<Trait>>();
        QueryBuilder::new(format!("INSERT INTO {gs}_trait_table (name, description)"))
            .push_values(complete_traits, |mut b, el| {
                b.push_bind(el.name).push_bind(el.description);
            })
            .push(" ON CONFLICT DO NOTHING")
            .build()
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_weapon_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_weapon_association_table (weapon_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_weapon_attack_effect_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    effects: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !effects.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_weapon_attack_effect_table (weapon_id, effect_name) "
        ))
        .push_values(effects, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_shield_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_shield_association_table (shield_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_armor_association_table (armor_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_item_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_item_association_table (item_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_cr_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_creature_association_table (creature_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_language_and_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    languages: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in languages {
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_language_table (name) VALUES ($1) ON CONFLICT DO NOTHING",
        )))
        .bind(el)
        .execute(&mut **conn)
        .await?;
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_language_creature_association_table (creature_id, language_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )))
        .bind(id)
        .bind(el)
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_immunity_and_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    immunities: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in immunities {
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_immunity_table (name) VALUES ($1) ON CONFLICT DO NOTHING"
        )))
        .bind(el)
        .execute(&mut **conn)
        .await?;
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_immunity_creature_association_table (creature_id, immunity_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )))
        .bind(id)
        .bind(el)
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_sense_and_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    senses: &Vec<Sense>,
    id: i64,
) -> Result<bool> {
    for el in senses {
        let sense_id: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_sense_table (name, acuity) VALUES ($1, $2) RETURNING id"
        )))
        .bind(&el.name)
        .bind(&el.acuity)
        .fetch_one(&mut **conn)
        .await?;
        if let Some(range) = el.range.clone() {
            let range_id = insert_range(conn, gs, range).await?;
            sqlx::query(sqlx::AssertSqlSafe(format!(
                "INSERT INTO {gs}_range_sense_association_table (range_id, sense_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )))
            .bind(range_id)
            .bind(sense_id)
            .execute(&mut **conn)
            .await?;
        }

        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_sense_creature_association_table (creature_id, sense_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )))
        .bind(id)
        .bind(sense_id)
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_speeds(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    speed: &HashMap<String, i64>,
    id: i64,
) -> Result<bool> {
    if !speed.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_speed_table (creature_id, name, value) "
        ))
        .push_values(speed, |mut b, (speed_type, speed_value)| {
            b.push_bind(id).push_bind(speed_type).push_bind(speed_value);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }

    Ok(true)
}

async fn insert_resistances(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    resistances: &Vec<Resistance>,
    id: i64,
) -> Result<bool> {
    for res in resistances {
        let res_id: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_resistance_table (creature_id, name, value) \
             VALUES ($1, $2, $3) RETURNING id"
        )))
        .bind(id)
        .bind(&res.name)
        .bind(res.value)
        .fetch_one(&mut **conn)
        .await?;

        insert_resistance_double_vs(conn, gs, res_id, res.double_vs.clone()).await?;
        insert_resistance_exception_vs(conn, gs, res_id, res.exceptions.clone()).await?;
    }

    Ok(true)
}

async fn insert_resistance_double_vs(
    conn: &mut Transaction<'_, Postgres>,
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
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }

    Ok(true)
}

async fn insert_resistance_exception_vs(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    res_id: i64,
    exception_vs: Vec<String>,
) -> Result<bool> {
    if !exception_vs.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_resistance_exception_vs_table (resistance_id, vs_name) "
        ))
        .push_values(exception_vs, |mut b, el| {
            b.push_bind(res_id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_weaknesses(
    conn: &mut Transaction<'_, Postgres>,
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
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }

    Ok(true)
}

async fn insert_item(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    item: &BybeItem,
) -> Result<i64> {
    let size = item.size.to_string();
    let rarity = item.rarity.to_string();

    // Use SAVEPOINT to recover if item_stats UNIQUE constraint fires on a different foundry_id
    sqlx::query("SAVEPOINT item_insert")
        .execute(&mut **conn)
        .await?;

    let insert_result = sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_item_table (
            foundry_id, name, bulk, base_item, category, description,
            hardness, hp, level, price, usage, item_group, item_type,
            is_derived, material_grade, material_type, number_of_uses,
            license, remaster, source, rarity, size, status
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12, $13,
            $14, $15, $16, $17,
            $18, $19, $20, $21, $22, $23
        ) ON CONFLICT (foundry_id) DO UPDATE SET
            name = excluded.name,
            bulk = excluded.bulk,
            base_item = excluded.base_item,
            category = excluded.category,
            description = excluded.description,
            hardness = excluded.hardness,
            hp = excluded.hp,
            level = excluded.level,
            price = excluded.price,
            usage = excluded.usage,
            item_group = excluded.item_group,
            item_type = excluded.item_type,
            is_derived = excluded.is_derived,
            material_grade = excluded.material_grade,
            material_type = excluded.material_type,
            number_of_uses = excluded.number_of_uses,
            license = excluded.license,
            remaster = excluded.remaster,
            source = excluded.source,
            rarity = excluded.rarity,
            size = excluded.size,
            status = excluded.status"
    )))
    .bind(&item.foundry_id)
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
    .bind(item.status.to_string())
    .execute(&mut **conn)
    .await;

    if insert_result.is_err() {
        sqlx::query("ROLLBACK TO SAVEPOINT item_insert")
            .execute(&mut **conn)
            .await?;
    }
    sqlx::query("RELEASE SAVEPOINT item_insert")
        .execute(&mut **conn)
        .await?;

    // Try by foundry_id first; if the item_stats constraint deduplicated it, fall back to name.
    let id_by_foundry_id: Option<i64> = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT id FROM {gs}_item_table WHERE foundry_id = $1"
    )))
    .bind(&item.foundry_id)
    .fetch_optional(&mut **conn)
    .await?;

    Ok(if let Some(id) = id_by_foundry_id {
        id
    } else {
        sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT id FROM {gs}_item_table WHERE name = $1"
        )))
        .bind(&item.name)
        .fetch_one(&mut **conn)
        .await?
    })
}

async fn insert_item_creature_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    item_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_item_creature_association_table \
            (item_id, creature_id, quantity) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
    )))
    .bind(item_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn insert_creature(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    cr: &BybeCreature,
) -> Result<i64> {
    let size = cr.size.to_string();
    let rarity = cr.rarity.to_string();
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_creature_table (
            foundry_id, name, aon_id,
            charisma, constitution, dexterity, intelligence, strength, wisdom,
            ac, hp, hp_detail, ac_detail, language_detail,
            level, license, remaster, source,
            initiative_ability, perception, perception_detail, vision,
            fortitude, reflex, will,
            fortitude_detail, reflex_detail, will_detail,
            rarity, size, cr_type, family, n_of_focus_points, status
        ) VALUES (
            $1, $2, $3,
            $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14,
            $15, $16, $17, $18,
            $19, $20, $21, $22,
            $23, $24, $25,
            $26, $27, $28,
            $29, $30, $31, $32, $33, $34
        ) ON CONFLICT (foundry_id) DO UPDATE SET
            name = excluded.name,
            aon_id = excluded.aon_id,
            charisma = excluded.charisma,
            constitution = excluded.constitution,
            dexterity = excluded.dexterity,
            intelligence = excluded.intelligence,
            strength = excluded.strength,
            wisdom = excluded.wisdom,
            ac = excluded.ac,
            hp = excluded.hp,
            hp_detail = excluded.hp_detail,
            ac_detail = excluded.ac_detail,
            language_detail = excluded.language_detail,
            level = excluded.level,
            license = excluded.license,
            remaster = excluded.remaster,
            source = excluded.source,
            initiative_ability = excluded.initiative_ability,
            perception = excluded.perception,
            perception_detail = excluded.perception_detail,
            vision = excluded.vision,
            fortitude = excluded.fortitude,
            reflex = excluded.reflex,
            will = excluded.will,
            fortitude_detail = excluded.fortitude_detail,
            reflex_detail = excluded.reflex_detail,
            will_detail = excluded.will_detail,
            rarity = excluded.rarity,
            size = excluded.size,
            cr_type = excluded.cr_type,
            family = excluded.family,
            n_of_focus_points = excluded.n_of_focus_points,
            status = excluded.status"
    )))
    .bind(&cr.foundry_id)
    .bind(&cr.name)
    .bind(None::<i32>) // aon_id, fetched later
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
    .bind(None::<String>) // cr_type
    .bind(None::<String>) // family
    .bind(cr.n_of_focus_points)
    .bind(cr.status.to_string())
    .execute(&mut **conn)
    .await?;
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT id FROM {gs}_creature_table WHERE foundry_id = $1"
    )))
    .bind(&cr.foundry_id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_shield(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    shield: &BybeShield,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_shield_table (ac_bonus, n_of_reinforcing_runes, speed_penalty, base_item_id) \
         VALUES ($1, $2, $3, $4) RETURNING id"
    )))
    .bind(shield.ac_bonus)
    .bind(shield.n_of_reinforcing_runes)
    .bind(shield.speed_penalty)
    .bind(item_id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_shield_creature_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    shield_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_shield_creature_association_table \
            (shield_id, creature_id, quantity) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )))
    .bind(shield_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn insert_weapon(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    wp: &BybeWeapon,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_weapon_table \
         (to_hit_bonus, splash_dmg, n_of_potency_runes, n_of_striking_runes, \
          reload, weapon_type, base_item_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"
    )))
    .bind(wp.to_hit_bonus)
    .bind(wp.splash_dmg)
    .bind(wp.n_of_potency_runes)
    .bind(wp.n_of_striking_runes)
    .bind(&wp.reload)
    .bind(&wp.weapon_type)
    .bind(item_id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_weapon_damage(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    dmg_data: &Vec<WeaponDamageData>,
    wp_id: i64,
) -> Result<()> {
    for el in dmg_data {
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO {gs}_weapon_damage_table \
             (bonus_dmg, dmg_type, number_of_dice, die_size, weapon_id) \
             VALUES ($1, $2, $3, $4, $5)"
        )))
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
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    weapon_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_weapon_creature_association_table \
            (weapon_id, creature_id, quantity) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )))
    .bind(weapon_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn insert_armor(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    armor: &BybeArmor,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_armor_table \
         (ac_bonus, check_penalty, dex_cap, n_of_potency_runes, n_of_resilient_runes, \
          speed_penalty, strength_required, base_item_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"
    )))
    .bind(armor.ac_bonus)
    .bind(armor.check_penalty)
    .bind(armor.dex_cap)
    .bind(armor.n_of_potency_runes)
    .bind(armor.n_of_resilient_runes)
    .bind(armor.speed_penalty)
    .bind(armor.strength_required)
    .bind(item_id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_armor_creature_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    armor_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_armor_creature_association_table \
            (armor_id, creature_id, quantity) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )))
    .bind(armor_id)
    .bind(cr_id)
    .bind(quantity)
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn insert_spellcasting_entry(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    spellcasting_entry: &SpellCastingEntry,
    id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_spellcasting_entry_table (
            spellcasting_name, is_spellcasting_flexible, type_of_spellcaster,
            spellcasting_dc_mod, spellcasting_atk_mod, spellcasting_tradition,
            heighten_level, creature_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"
    )))
    .bind(&spellcasting_entry.name)
    .bind(spellcasting_entry.is_flexible)
    .bind(&spellcasting_entry.type_of_spellcaster)
    .bind(spellcasting_entry.dc_modifier)
    .bind(spellcasting_entry.atk_modifier)
    .bind(&spellcasting_entry.tradition)
    .bind(spellcasting_entry.heighten_level)
    .bind(id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_spell_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_spell_association_table (spell_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_tradition_and_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    tradition: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !tradition.is_empty() {
        QueryBuilder::new(format!("INSERT INTO {gs}_tradition_table (name) "))
            .push_values(tradition, |mut b, el| {
                b.push_bind(el);
            })
            .push(" ON CONFLICT DO NOTHING")
            .build()
            .execute(&mut **conn)
            .await?;

        QueryBuilder::new(format!(
            "INSERT INTO {gs}_tradition_spell_association_table (spell_id, tradition_id) "
        ))
        .push_values(tradition, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_spell(
    conn: &mut Transaction<'_, Postgres>,
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
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_spell_table (
            name, area_type, area_value, counteraction,
            saving_throw, basic_saving_throw, sustained, duration,
            level, target, actions,
            license, remaster, source, rarity,
            slot, creature_id, spellcasting_entry_id
        ) VALUES (
            $1, $2, $3, $4,
            $5, $6, $7, $8,
            $9, $10, $11, $12,
            $13, $14, $15, $16,
            $17, $18
        ) RETURNING id"
    )))
    .bind(&spell.name)
    .bind(area_type)
    .bind(area_value)
    .bind(spell.counteraction)
    .bind(save_throw)
    .bind(save_throw_mod)
    .bind(spell.sustained)
    .bind(&spell.duration)
    .bind(spell.level)
    .bind(&spell.target)
    .bind(&spell.actions)
    .bind(&spell.publication_info.license)
    .bind(spell.publication_info.remastered)
    .bind(&spell.publication_info.source)
    .bind(&spell.traits.rarity)
    .bind(slot)
    .bind(cr_id)
    .bind(spellcasting_entry_id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_action(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    action: &Action,
) -> Result<i64> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_action_table (
            name, action_type, n_of_actions, category, description,
            license, remaster, source, slug, rarity
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id"
    )))
    .bind(&action.name)
    .bind(&action.action_type)
    .bind(action.n_of_actions)
    .bind(&action.category)
    .bind(&action.description)
    .bind(&action.publication_info.license)
    .bind(action.publication_info.remastered)
    .bind(&action.publication_info.source)
    .bind(&action.slug)
    .bind(action.traits.rarity.to_string())
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_action_trait_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    if !traits.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_trait_action_association_table (action_id, trait_id) "
        ))
        .push_values(traits, |mut b, el| {
            b.push_bind(id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_skill(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    skill: &Skill,
    cr_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_skill_table (
            name, description, modifier, proficiency,
            license, remaster, source, creature_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"
    )))
    .bind(&skill.name)
    .bind(&skill.description)
    .bind(skill.modifier)
    .bind(skill.proficiency)
    .bind(&skill.publication_info.license)
    .bind(skill.publication_info.remastered)
    .bind(&skill.publication_info.source)
    .bind(cr_id)
    .fetch_one(&mut **conn)
    .await?)
}

async fn insert_skill_modifier_variant_table(
    conn: &mut Transaction<'_, Postgres>,
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
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    runes: &Vec<String>,
) -> Result<bool> {
    if !runes.is_empty() {
        QueryBuilder::new(format!("INSERT INTO {gs}_rune_table (name) "))
            .push_values(runes, |mut b, el| {
                b.push_bind(el);
            })
            .push(" ON CONFLICT DO NOTHING")
            .build()
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_range(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    range: RangeData,
) -> Result<i64> {
    let range_id: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "INSERT INTO {gs}_range_table (value, increment, max)
         VALUES ($1, $2, $3)
         ON CONFLICT (value, increment, max) DO UPDATE
             SET value = EXCLUDED.value
         RETURNING id"
    )))
    .bind(range.value)
    .bind(range.increment)
    .bind(range.max)
    .fetch_one(&mut **conn)
    .await?;
    Ok(range_id)
}

async fn insert_weapon_rune_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    runes: &Vec<String>,
    wp_id: i64,
) -> Result<bool> {
    if !runes.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_rune_weapon_association_table (weapon_id, rune_id) "
        ))
        .push_values(runes, |mut b, el| {
            b.push_bind(wp_id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor_rune_association(
    conn: &mut Transaction<'_, Postgres>,
    gs: &GameSystem,
    runes: &Vec<String>,
    arm_id: i64,
) -> Result<bool> {
    if !runes.is_empty() {
        QueryBuilder::new(format!(
            "INSERT INTO {gs}_rune_armor_association_table (armor_id, rune_id) "
        ))
        .push_values(runes, |mut b, el| {
            b.push_bind(arm_id).push_bind(el);
        })
        .push(" ON CONFLICT DO NOTHING")
        .build()
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}
