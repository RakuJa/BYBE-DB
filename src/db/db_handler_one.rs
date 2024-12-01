use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeShield, BybeWeapon, WeaponDamageData};
use crate::schema::source_schema::creature::item::action::Action;
use crate::schema::source_schema::creature::item::skill::Skill;
use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::sense::Sense;
use anyhow::Result;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{query_file, Sqlite, SqlitePool, Transaction};
use std::collections::HashMap;
use std::str::FromStr;

pub async fn connect(db_path: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(db_path)?.create_if_missing(true);
    Ok(SqlitePool::connect_with(options).await?)
}

pub async fn insert_item_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    item: &BybeItem,
    cr_id: Option<i64>,
) -> Result<i64> {
    let item_id = insert_item(conn, item).await?;
    insert_traits(conn, &item.traits).await?;
    insert_item_trait_association(conn, &item.traits, item_id).await?;
    if let Some(creature_id) = cr_id {
        insert_item_creature_association(conn, item_id, creature_id, item.quantity).await?;
    }
    Ok(item_id)
}

pub async fn insert_shield_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    shield: &BybeShield,
    cr_id: Option<i64>,
) -> Result<i64> {
    // Don't creature useless links between item & creature.
    // Since item is as generic as possible the specializations
    // (weapon, armor, etc) should have a separate association table
    let item_id = insert_item_to_db(conn, &shield.item_core, None).await?;

    let shield_id = insert_shield(conn, shield, item_id).await?;

    if let Some(creature_id) = cr_id {
        insert_weapon_creature_association(conn, shield_id, creature_id, shield.item_core.quantity)
            .await?;
    }
    insert_shield_trait_association(conn, &shield.item_core.traits, shield_id).await?;

    Ok(shield_id)
}

pub async fn insert_weapon_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    wp: &BybeWeapon,
    cr_id: Option<i64>,
) -> Result<i64> {
    // Don't creature useless links between item & creature.
    // Since item is as generic as possible the specializations
    // (weapon, armor, etc) should have a separate association table
    let item_id = insert_item_to_db(conn, &wp.item_core, None).await?;

    let wp_id = insert_weapon(conn, wp, item_id).await?;
    if let Some(creature_id) = cr_id {
        insert_weapon_creature_association(conn, wp_id, creature_id, wp.item_core.quantity).await?;
    }

    insert_weapon_damage(conn, &wp.damage_data, wp_id).await?;

    insert_runes(conn, &wp.property_runes).await?;
    insert_weapon_rune_association(conn, &wp.property_runes, wp_id).await?;

    insert_weapon_trait_association(conn, &wp.item_core.traits, wp_id).await?;
    Ok(wp_id)
}

pub async fn insert_armor_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    armor: &BybeArmor,
    cr_id: Option<i64>,
) -> Result<i64> {
    // Don't creature useless links between item & creature.
    // Since item is as generic as possible the specializations
    // (weapon, armor, etc) should have a separate association table
    let item_id = insert_item_to_db(conn, &armor.item_core, None).await?;

    let arm_id = insert_armor(conn, armor, item_id).await?;

    if let Some(creature_id) = cr_id {
        insert_armor_creature_association(conn, arm_id, creature_id, armor.item_core.quantity)
            .await?;
    }

    insert_runes(conn, &armor.property_runes).await?;
    insert_armor_rune_association(conn, &armor.property_runes, arm_id).await?;

    insert_armor_trait_association(conn, &armor.item_core.traits, arm_id).await?;
    Ok(arm_id)
}

pub async fn insert_creature_to_db<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    cr: BybeCreature,
) -> Result<bool> {
    let cr_id = insert_creature(conn, &cr).await?;
    insert_traits(conn, &cr.traits).await?;
    insert_cr_trait_association(conn, &cr.traits, cr_id).await?;
    insert_language_and_association(conn, &cr.languages, cr_id).await?;
    insert_immunity_and_association(conn, &cr.immunities, cr_id).await?;
    insert_sense_and_association(conn, &cr.senses, cr_id).await?;
    insert_speeds(conn, &cr.speed, cr_id).await?;
    insert_weaknesses(conn, &cr.weaknesses, cr_id).await?;
    insert_resistances(conn, &cr.resistances, cr_id).await?;
    for el in &cr.weapons {
        insert_weapon_to_db(conn, el, Some(cr_id)).await?;
    }
    for el in &cr.armors {
        insert_armor_to_db(conn, el, Some(cr_id)).await?;
    }
    for el in &cr.spells {
        let spell_id = insert_spell(conn, el, cr_id).await?;
        insert_traits(conn, &el.traits.traits).await?;
        insert_spell_trait_association(conn, &el.traits.traits, spell_id).await?;
        insert_tradition_and_association(conn, &el.traits.traditions, spell_id).await?;
    }
    for el in &cr.actions {
        let action_id = insert_action(conn, el, cr_id).await?;
        insert_traits(conn, &el.traits.traits).await?;
        insert_action_trait_association(conn, &el.traits.traits, action_id).await?;
    }
    for el in &cr.skills {
        let skill_id = insert_skill(conn, el, cr_id).await?;
        insert_skill_modifier_variant_table(conn, &el.variant_label, cr_id, skill_id).await?;
    }
    for el in &cr.items {
        insert_item_to_db(conn, el, Some(cr_id)).await?;
    }
    Ok(true)
}

pub async fn update_with_aon_data<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<bool> {
    query_file!("src/db/raw_queries/update_mon_w_aon_data.sql")
        .execute(&mut **conn)
        .await?;
    query_file!("src/db/raw_queries/update_npc_w_aon_data.sql")
        .execute(&mut **conn)
        .await?;
    Ok(true)
}

pub async fn insert_scales_values_to_db<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<bool> {
    query_file!("src/db/raw_queries/scales.sql")
        .execute(&mut **conn)
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

async fn insert_weapon_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT OR IGNORE INTO TRAIT_WEAPON_ASSOCIATION_TABLE
            (weapon_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_shield_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT OR IGNORE INTO TRAIT_SHIELD_ASSOCIATION_TABLE
            (shield_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_armor_trait_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    traits: &Vec<String>,
    id: i64,
) -> Result<bool> {
    for el in traits {
        sqlx::query!(
            "INSERT OR IGNORE INTO TRAIT_ARMOR_ASSOCIATION_TABLE
            (armor_id, trait_id) VALUES ($1, $2)",
            id,
            el
        )
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
            "INSERT OR IGNORE INTO TRAIT_ITEM_ASSOCIATION_TABLE
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
            "INSERT OR IGNORE INTO TRAIT_CREATURE_ASSOCIATION_TABLE
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
            "INSERT OR IGNORE INTO LANGUAGE_CREATURE_ASSOCIATION_TABLE
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
    senses: &Vec<Sense>,
    id: i64,
) -> Result<bool> {
    for el in senses {
        let sense_id = sqlx::query!(
            "INSERT OR IGNORE INTO SENSE_TABLE (id, name, range, acuity) VALUES ($1, $2, $3, $4)",
            None::<i64>,
            el.name,
            el.range,
            el.acuity
        )
        .execute(&mut **conn)
        .await?
        .last_insert_rowid();
        sqlx::query!(
            "INSERT OR IGNORE INTO SENSE_CREATURE_ASSOCIATION_TABLE
            (creature_id, sense_id) VALUES ($1, $2)",
            id,
            sense_id
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
    // we check if a similar base item is already present
    // if it is then we return the id without inserting a new entry
    match sqlx::query!(
        "
        INSERT INTO ITEM_TABLE VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21
        );
    ",
        None::<i64>, // id, autoincrement
        item.name,
        item.bulk,
        item.base_item,
        item.category,
        item.description,
        item.hardness,
        item.hp,
        item.level,
        item.price,
        item.usage,
        item.group,
        item.item_type,
        item.material_grade,
        item.material_type,
        item.number_of_uses,
        item.license,
        item.remaster,
        item.source,
        rarity,
        size
    )
    .execute(&mut **conn)
    .await
    {
        Ok(r) => Ok(r.last_insert_rowid()),
        Err(_) => {
            let x = sqlx::query!(
                "SELECT id FROM ITEM_TABLE WHERE
                name = $1 AND bulk =$2 AND description = $3 AND hardness = $4 AND
                hp = $5 AND level = $6 AND price = $7 AND item_type = $8 AND
                license = $9 AND remaster = $10 AND source = $11 AND
                rarity = $12 AND size = $13
                ",
                item.name,
                item.bulk,
                item.description,
                item.hardness,
                item.hp,
                item.level,
                item.price,
                item.item_type,
                item.license,
                item.remaster,
                item.source,
                rarity,
                size
            )
            .fetch_one(&mut **conn)
            .await?
            .id;
            Ok(x)
        }
    }
}

async fn insert_item_creature_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    item_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query!(
        "INSERT OR IGNORE INTO ITEM_CREATURE_ASSOCIATION_TABLE
            (item_id, creature_id, quantity) VALUES ($1, $2, $3)",
        item_id,
        cr_id,
        quantity
    )
    .execute(&mut **conn)
    .await?;
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
                $31, $32, $33, $34, $35, $36, $37, $38, $39
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
        cr.vision,
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
// SHIELD CORE START

async fn insert_shield<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    shield: &BybeShield,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query!(
        "
            INSERT INTO SHIELD_TABLE VALUES (
                $1, $2, $3, $4, $5
            )",
        None::<i64>, // id, autoincrement
        shield.ac_bonus,
        shield.n_of_reinforcing_runes,
        shield.speed_penalty,
        item_id,
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_shield_creature_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    shield_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query!(
        "INSERT OR IGNORE INTO SHIELD_CREATURE_ASSOCIATION_TABLE
            (shield_id, creature_id, quantity) VALUES ($1, $2, $3)",
        shield_id,
        cr_id,
        quantity
    )
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

// WEAPON CORE START
async fn insert_weapon<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    wp: &BybeWeapon,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query!(
        "
        INSERT INTO WEAPON_TABLE VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9
        )",
        None::<i64>, // id, autoincrement
        wp.to_hit_bonus,
        wp.splash_dmg,
        wp.n_of_potency_runes,
        wp.n_of_striking_runes,
        wp.range,
        wp.reload,
        wp.weapon_type,
        item_id,
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}

async fn insert_weapon_damage<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    dmg_data: &Vec<WeaponDamageData>,
    wp_id: i64,
) -> Result<()> {
    for el in dmg_data {
        sqlx::query!(
            "
        INSERT INTO WEAPON_DAMAGE_TABLE VALUES (
            $1, $2, $3, $4, $5, $6
        )",
            None::<i64>, // id, autoincrement
            el.bonus_dmg,
            el.dmg_type,
            el.n_of_dice,
            el.die_size,
            wp_id,
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(())
}

async fn insert_weapon_creature_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    weapon_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query!(
        "INSERT OR IGNORE INTO WEAPON_CREATURE_ASSOCIATION_TABLE
            (weapon_id, creature_id, quantity) VALUES ($1, $2, $3)",
        weapon_id,
        cr_id,
        quantity
    )
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

// WEAPON CORE END

// ARMOR CORE START

async fn insert_armor<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    armor: &BybeArmor,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query!(
        "
            INSERT INTO ARMOR_TABLE VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )",
        None::<i64>, // id, autoincrement
        armor.ac_bonus,
        armor.check_penalty,
        armor.dex_cap,
        armor.n_of_potency_runes,
        armor.n_of_resilient_runes,
        armor.speed_penalty,
        armor.strength_required,
        item_id,
    )
    .execute(&mut **conn)
    .await?
    .last_insert_rowid())
}
async fn insert_armor_creature_association<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    armor_id: i64,
    cr_id: i64,
    quantity: i64,
) -> Result<bool> {
    sqlx::query!(
        "INSERT OR IGNORE INTO ARMOR_CREATURE_ASSOCIATION_TABLE
            (armor_id, creature_id, quantity) VALUES ($1, $2, $3)",
        armor_id,
        cr_id,
        quantity
    )
    .execute(&mut **conn)
    .await?;
    Ok(true)
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
            "INSERT OR IGNORE INTO TRAIT_SPELL_ASSOCIATION_TABLE
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
            "INSERT OR IGNORE INTO TRADITION_SPELL_ASSOCIATION_TABLE
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
            "INSERT OR IGNORE INTO TRAIT_ACTION_ASSOCIATION_TABLE
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
            "INSERT INTO CREATURE_SKILL_LABEL_TABLE
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
            "INSERT OR IGNORE INTO RUNE_WEAPON_ASSOCIATION_TABLE
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
            "INSERT OR IGNORE INTO RUNE_ARMOR_ASSOCIATION_TABLE
            (armor_id, rune_id) VALUES ($1, $2)",
            arm_id,
            el
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}
