use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeWeapon};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use anyhow::Result;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{query_file, Sqlite, SqlitePool, Transaction};
use std::collections::HashMap;
use std::str::FromStr;

pub async fn connect(db_path: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(db_path)?.create_if_missing(true);
    Ok(SqlitePool::connect_with(options).await?)
}

pub async fn insert_item_to_db(conn: &SqlitePool, item: BybeItem) -> Result<()> {
    let mut tx: Transaction<Sqlite> = conn.begin().await?;
    let item_id = insert_item(&mut tx, &item).await?;
    insert_traits(&mut tx, &item.traits).await?;
    insert_item_trait_association(&mut tx, &item.traits, item_id).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn insert_weapon_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    wp: &BybeWeapon,
) -> Result<i64> {
    let wp_id = insert_weapon(conn, wp).await?;
    insert_traits(conn, &wp.item_core.traits).await?;
    insert_runes(conn, &wp.property_runes).await?;

    insert_wp_trait_association(conn, &wp.item_core.traits, wp_id).await?;
    insert_weapon_rune_association(conn, &wp.property_runes, wp_id).await?;
    Ok(wp_id)
}

pub async fn insert_armor_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    armor: &BybeArmor,
) -> Result<i64> {
    let arm_id = insert_armor(conn, armor).await?;
    insert_traits(conn, &armor.item_core.traits).await?;
    insert_runes(conn, &armor.property_runes).await?;
    insert_armor_trait_association(conn, &armor.item_core.traits, arm_id).await?;
    insert_armor_rune_association(conn, &armor.property_runes, arm_id).await?;
    Ok(arm_id)
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
        let wp_id = insert_weapon_to_db(&mut tx, el).await?;
        insert_weapon_creature_association(&mut tx, wp_id, cr_id).await?;
    }
    for el in &cr.spells {
        let spell_id = insert_spell(&mut tx, el, cr_id).await?;
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

async fn insert_item_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT OR IGNORE INTO TRAIT_ITEM_ASSOCIATION_TABLE \
            (item_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
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
            "INSERT OR IGNORE INTO TRAIT_CREATURE_ASSOCIATION_TABLE \
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
            "INSERT OR IGNORE INTO LANGUAGE_CREATURE_ASSOCIATION_TABLE \
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
            "INSERT OR IGNORE INTO IMMUNITY_CREATURE_ASSOCIATION_TABLE \
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

async fn insert_item<'a>(conn: &mut Transaction<'a, Sqlite>, item: &BybeItem) -> Result<i64> {
    let size = item.size.to_string();
    let rarity = item.rarity.to_string();
    let x = sqlx::query!(
        "
        INSERT INTO ITEM_TABLE VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21
        );
    ",
        None::<i64>, // id, autoincrement
        item.name,
        item.bulk,
        item.quantity,
        item.base_item,
        item.category,
        item.description,
        item.hardness,
        item.hp,
        item.level,
        item.price,
        item.usage,
        item.item_type,
        item.material_grade,
        item.material_type,
        item.number_of_uses,
        item.license,
        item.remaster,
        item.source,
        rarity,
        size
    );
    Ok(x.execute(&mut **conn).await?.last_insert_rowid())
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
                $31, $32, $33, $34, $35, $36, $37, $38
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
        cr.n_of_focus_points,
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

async fn insert_weapon<'a>(conn: &mut Transaction<'a, Sqlite>, wp: &BybeWeapon) -> Result<i64> {
    let size = wp.item_core.size.to_string();
    let rarity = wp.item_core.rarity.to_string();
    Ok(sqlx::query!(
        "
            INSERT INTO WEAPON_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                $31
            )",
        None::<i64>, // id, autoincrement
        wp.item_core.name,
        wp.item_core.bulk,
        wp.item_core.quantity,
        wp.item_core.base_item,
        wp.item_core.category,
        wp.item_core.description,
        wp.item_core.hardness,
        wp.item_core.hp,
        wp.item_core.level,
        wp.item_core.price,
        wp.item_core.usage,
        wp.item_core.item_type,
        wp.item_core.material_grade,
        wp.item_core.material_type,
        wp.item_core.number_of_uses,
        wp.item_core.license,
        wp.item_core.remaster,
        wp.item_core.source,
        rarity,
        size,
        wp.bonus_dmg,
        wp.to_hit_bonus,
        wp.dmg_type,
        wp.number_of_dice,
        wp.die_size,
        wp.splash_dmg,
        wp.n_of_potency_runes,
        wp.n_of_striking_runes,
        wp.range,
        wp.reload
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_weapon_creature_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    wp_id: i64,
    cr_id: i64,
) -> Result<bool> {
    sqlx::query!(
        "INSERT INTO WEAPON_CREATURE_ASSOCIATION_TABLE
            (weapon_id, creature_id) VALUES ($1, $2)",
        wp_id,
        cr_id
    )
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

// WEAPON CORE END

// ARMOR CORE START

async fn insert_armor_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT INTO TRAIT_ARMOR_ASSOCIATION_TABLE \
            (armor_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor<'a>(conn: &mut Transaction<'a, Sqlite>, armor: &BybeArmor) -> Result<i64> {
    let size = armor.item_core.size.to_string();
    let rarity = armor.item_core.rarity.to_string();
    Ok(sqlx::query!(
        "
            INSERT INTO ARMOR_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28
            )",
        None::<i64>, // id, autoincrement
        armor.item_core.name,
        armor.item_core.bulk,
        armor.item_core.quantity,
        armor.item_core.base_item,
        armor.item_core.category,
        armor.item_core.description,
        armor.item_core.hardness,
        armor.item_core.hp,
        armor.item_core.level,
        armor.item_core.price,
        armor.item_core.usage,
        armor.item_core.item_type,
        armor.item_core.material_grade,
        armor.item_core.material_type,
        armor.item_core.number_of_uses,
        armor.item_core.license,
        armor.item_core.remaster,
        armor.item_core.source,
        rarity,
        size,
        armor.ac_bonus,
        armor.check_penalty,
        armor.dex_cap,
        armor.n_of_potency_runes,
        armor.n_of_resilient_runes,
        armor.speed_penalty,
        armor.strength_required,
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

// ARMOR CORE END
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

async fn insert_spell<'a>(
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

async fn insert_runes<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
) -> Result<bool> {
    for el in traits {
        sqlx::query!("INSERT OR IGNORE INTO RUNE_TABLE (name) VALUES ($1)", el)
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_weapon_rune_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    runes: &Vec<String>,
    wp_id: i64,
) -> Result<bool> {
    for el in runes {
        sqlx::query!(
            "INSERT OR IGNORE INTO RUNE_WEAPON_ASSOCIATION_TABLE \
            (weapon_id, rune_id) VALUES ($1, $2)",
            wp_id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor_rune_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    runes: &Vec<String>,
    arm_id: i64,
) -> Result<bool> {
    for el in runes {
        sqlx::query!(
            "INSERT OR IGNORE INTO RUNE_ARMOR_ASSOCIATION_TABLE \
            (armor_id, rune_id) VALUES ($1, $2)",
            arm_id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}
