use crate::schema::bybe_creature::BybeCreature;
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::weapon::Weapon;
use anyhow::Result;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{query_file, Sqlite, SqlitePool, Transaction};
use std::collections::HashMap;
use std::str::FromStr;

pub async fn connect(db_path: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(db_path)?.create_if_missing(true);
    Ok(SqlitePool::connect_with(options).await?)
}

pub async fn insert_creature_to_db(conn: &SqlitePool, cr: BybeCreature) -> Result<bool> {
    let mut tx: Transaction<Sqlite> = conn.begin().await?;
    let cr_id = insert_creature(&mut tx, &cr).await?;
    insert_traits(&mut tx, &cr.traits).await?;
    insert_cr_trait_association(&mut tx, &cr.traits, cr_id).await?;
    insert_language_and_association(&mut tx, &cr.languages, cr_id).await?;
    insert_immunity_and_association(&mut tx, &cr.immunities, cr_id).await?;
    insert_sense_and_association(&mut tx, &cr.senses, cr_id).await?;
    insert_speeds(&mut tx, &cr.speed, cr_id).await?;
    insert_weaknesses(&mut tx, &cr.weaknesses, cr_id).await?;
    insert_resistances(&mut tx, &cr.resistances, cr_id).await?;
    for el in &cr.weapons {
        let wp_id = insert_weapon(&mut tx, el, cr_id).await?;
        insert_traits(&mut tx, &el.traits.traits).await?;
        insert_wp_trait_association(&mut tx, &el.traits.traits, wp_id).await?;
    }
    for el in &cr.spells {
        let spell_id = insert_spells(&mut tx, el, cr_id).await?;
        insert_traits(&mut tx, &el.traits.traits).await?;
        insert_spell_trait_association(&mut tx, &el.traits.traits, spell_id).await?;
        insert_tradition_and_association(&mut tx, &el.traits.traditions, spell_id).await?;
    }
    for el in &cr.actions {
        let action_id = insert_action(&mut tx, el, cr_id).await?;
        insert_traits(&mut tx, &el.traits.traits).await?;
        insert_action_trait_association(&mut tx, &el.traits.traits, action_id).await?;
    }
    for el in &cr.skills {
        let skill_id = insert_skill(&mut tx, el, cr_id).await?;
        insert_skill_modifier_variant_table(&mut tx, &el.variant_label, cr_id, skill_id).await?;
    }
    tx.commit().await?;

    Ok(true)
}

pub async fn update_with_aon_data(conn: &SqlitePool) -> Result<bool> {
    query_file!("src/db/raw_queries/update_mon_w_aon_data.sql")
        .execute(conn)
        .await?;
    query_file!("src/db/raw_queries/update_npc_w_aon_data.sql")
        .execute(conn)
        .await?;
    Ok(true)
}

pub async fn insert_scales_values_to_db(conn: &SqlitePool) -> Result<bool> {
    query_file!("src/db/raw_queries/scales.sql")
        .execute(conn)
        .await?;
    Ok(true)
}

async fn insert_traits<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
) -> Result<bool> {
    for el in traits {
        sqlx::query!("INSERT OR IGNORE INTO TRAIT_TABLE (name) VALUES ($1)", el)
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_cr_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT INTO TRAIT_CREATURE_ASSOCIATION_TABLE \
            (creature_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_language_and_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    languages: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in languages {
        sqlx::query!(
            "INSERT OR IGNORE INTO LANGUAGE_TABLE (name) VALUES ($1)",
            el
        )
        .execute(&mut **conn)
        .await?;
        sqlx::query!(
            "INSERT INTO LANGUAGE_CREATURE_ASSOCIATION_TABLE \
            (creature_id, language_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_immunity_and_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    immunities: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in immunities {
        sqlx::query!(
            "INSERT OR IGNORE INTO IMMUNITY_TABLE (name) VALUES ($1)",
            el
        )
        .execute(&mut **conn)
        .await?;
        sqlx::query!(
            "INSERT INTO IMMUNITY_CREATURE_ASSOCIATION_TABLE \
            (creature_id, immunity_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_sense_and_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    senses: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in senses {
        sqlx::query!("INSERT OR IGNORE INTO SENSE_TABLE (name) VALUES ($1)", el)
            .execute(&mut **conn)
            .await?;
        sqlx::query!(
            "INSERT OR IGNORE INTO SENSE_CREATURE_ASSOCIATION_TABLE \
            (creature_id, sense_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_speeds<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    speed: &HashMap<String, i64>,
    id: i64,
) -> Result<bool> {
    for (speed_type, speed_value) in speed {
        sqlx::query!(
            "INSERT INTO SPEED_TABLE (creature_id, name, value) VALUES ($1, $2, $3)",
            id,
            speed_type,
            speed_value
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_resistances<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    resistances: &HashMap<String, i64>,
    id: i64,
) -> Result<bool> {
    for (res_type, res_value) in resistances {
        sqlx::query!(
            "INSERT INTO RESISTANCE_TABLE (creature_id, name, value) VALUES ($1, $2, $3)",
            id,
            res_type,
            res_value
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_weaknesses<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    weaknesses: &HashMap<String, i64>,
    id: i64,
) -> Result<bool> {
    for (weak_type, weak_value) in weaknesses {
        sqlx::query!(
            "INSERT INTO WEAKNESS_TABLE (creature_id, name, value) VALUES ($1, $2, $3)",
            id,
            weak_type,
            weak_value
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_creature<'a>(conn: &mut Transaction<'a, Sqlite>, cr: &BybeCreature) -> Result<i64> {
    let size = cr.size.to_string();
    let rarity = cr.rarity.to_string();
    let spell_casting_entry = cr.spell_casting.clone();

    let spell_casting_name = spell_casting_entry.clone().map(|x| x.name);
    let spell_casting_flexible = spell_casting_entry.clone().and_then(|x| x.is_flexible);
    let spell_casting_type = spell_casting_entry.clone().map(|x| x.type_of_spell_caster);
    let spell_casting_dc = spell_casting_entry.clone().and_then(|x| x.dc_modifier);
    let spell_casting_atk_mod = spell_casting_entry.clone().and_then(|x| x.atk_modifier);
    let spell_casting_tradition = spell_casting_entry.clone().map(|x| x.tradition);

    Ok(sqlx::query!(
        "
            INSERT INTO CREATURE_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                $31, $32, $33, $34, $35, $36, $37
            )",
        None::<i64>, // id, autoincrement
        cr.name,
        None::<i64>, //aon_id, need to fetch it manually
        cr.charisma,
        cr.constitution,
        cr.dexterity,
        cr.intelligence,
        cr.strength,
        cr.wisdom,
        cr.ac,
        cr.hp,
        cr.hp_details,
        cr.ac_details,
        cr.languages_details,
        cr.level,
        cr.license,
        cr.remaster,
        cr.source,
        cr.initiative_ability,
        cr.perception_mod,
        cr.perception_details,
        cr.fortitude_mod,
        cr.reflex_mod,
        cr.will_mod,
        cr.fortitude_detail,
        cr.reflex_detail,
        cr.will_detail,
        rarity,
        size,
        None::<String>, // type, source says NPC always..
        None::<String>, // family, source does not have it
        spell_casting_name,
        spell_casting_flexible,
        spell_casting_type,
        spell_casting_dc,
        spell_casting_atk_mod,
        spell_casting_tradition,
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

// CREATURE CORE DONE
// WEAPON CORE START

async fn insert_wp_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT INTO TRAIT_WEAPON_ASSOCIATION_TABLE \
            (weapon_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_weapon<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    wp: &Weapon,
    cr_id: i64,
) -> Result<i64> {
    let (dmg_type, n_of_dices, die_size, bonus_dmg) = match wp.damage.clone() {
        Some(data) => (
            Some(data.dmg_type),
            Some(data.n_of_dices),
            Some(data.die_size),
            Some(data.bonus_dmg),
        ),
        None => (None, None, None, None),
    };

    Ok(sqlx::query!(
        "
            INSERT INTO WEAPON_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29
            )",
        None::<i64>, // id, autoincrement
        wp.name,
        wp.base_weapon,
        wp.to_hit_bonus,
        wp.bulk,
        wp.category,
        dmg_type,
        n_of_dices,
        die_size,
        bonus_dmg,
        wp.carry_type,
        wp.hands_held,
        wp.invested,
        wp.weapon_group,
        wp.hardness,
        wp.hp_max,
        wp.hp_curr,
        wp.level,
        wp.publication_info.license,
        wp.publication_info.remastered,
        wp.publication_info.source,
        wp.quantity,
        wp.range,
        wp.reload,
        wp.size,
        wp.traits.rarity,
        wp.usage,
        wp.weapon_type,
        cr_id
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

// WEAPON CORE END
// SPELL CORE START

async fn insert_spell_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT INTO TRAIT_SPELL_ASSOCIATION_TABLE \
            (spell_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_tradition_and_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    tradition: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in tradition {
        sqlx::query!(
            "INSERT OR IGNORE INTO TRADITION_TABLE (name) VALUES ($1)",
            el
        )
        .execute(&mut **conn)
        .await?;
        sqlx::query!(
            "INSERT INTO TRADITION_SPELL_ASSOCIATION_TABLE \
            (spell_id, tradition_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_spells<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    spell: &Spell,
    cr_id: i64,
) -> Result<i64> {
    let (area_type, area_value) = match spell.area.clone() {
        Some(data) => (Some(data.area_type), Some(data.area_value)),
        None => (None, None),
    };
    let (save_throw, save_throw_mod) = match spell.saving_throw.clone() {
        Some(data) => (Some(data.statistic), Some(data.basic)),
        None => (None, None),
    };
    Ok(sqlx::query!(
        "
            INSERT INTO SPELL_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18
            )",
        None::<i64>, // id, autoincrement
        spell.name,
        area_type,
        area_value,
        spell.counteraction,
        save_throw,
        save_throw_mod,
        spell.sustained,
        spell.duration,
        spell.level,
        spell.range,
        spell.target,
        spell.actions,
        spell.publication_info.license,
        spell.publication_info.remastered,
        spell.publication_info.source,
        spell.traits.rarity,
        cr_id
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

// ACTION

async fn insert_action<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    action: &Action,
    cr_id: i64,
) -> Result<i64> {
    Ok(sqlx::query!(
        "
            INSERT INTO ACTION_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )",
        None::<i64>, // id, autoincrement
        action.name,
        action.action_type,
        action.n_of_actions,
        action.category,
        action.description,
        action.publication_info.license,
        action.publication_info.remastered,
        action.publication_info.source,
        action.slug,
        action.traits.rarity,
        cr_id
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_action_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT INTO TRAIT_ACTION_ASSOCIATION_TABLE \
            (action_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_skill<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    skill: &Skill,
    cr_id: i64,
) -> Result<i64> {
    Ok(sqlx::query!(
        "
            INSERT INTO SKILL_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )",
        None::<i64>, // id, autoincrement
        skill.name,
        skill.description,
        skill.modifier,
        skill.proficiency,
        skill.publication_info.license,
        skill.publication_info.remastered,
        skill.publication_info.source,
        cr_id
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_skill_modifier_variant_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    skill_labels: &Vec<String>,
    cr_id: i64,
    skill_id: i64,
) -> Result<bool> {
    for el in skill_labels {
        sqlx::query!(
            "INSERT INTO CREATURE_SKILL_LABEL_TABLE \
            (creature_id, skill_id, skill_label) VALUES ($1, $2, $3)",
            cr_id,
            skill_id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}
