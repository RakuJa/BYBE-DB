use sqlx::{Sqlite, Transaction};

pub async fn init_creature_builder_tables<'a>(
    tx: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    init_ability_mod_scales(tx).await?;
    init_perception_scales(tx).await?;
    init_skill_scales(tx).await?;
    init_item_scales(tx).await?;
    init_ac_scales(tx).await?;
    init_saving_throw_scales(tx).await?;
    init_hp_scales(tx).await?;
    init_res_weak_scales(tx).await?;
    init_strike_bonus_scales(tx).await?;
    init_strike_dmg_scales(tx).await?;
    init_spell_dc_and_atk_scales(tx).await?;
    init_area_dmg_scales(tx).await?;
    Ok(true)
}

async fn init_ability_mod_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS ABILITY_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme INTEGER,
            high INTEGER NOT NULL,
            moderate INTEGER NOT NULL,
            low INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_perception_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS PERCEPTION_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme INTEGER NOT NULL,
            high INTEGER NOT NULL,
            moderate INTEGER NOT NULL,
            low INTEGER NOT NULL,
            terrible INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_skill_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SKILL_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme INTEGER NOT NULL,
            high INTEGER NOT NULL,
            moderate INTEGER NOT NULL,
            low_ub INTEGER NOT NULL,
            low_lb INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_item_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS ITEM_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cr_level TEXT UNIQUE NOT NULL,
            safe_item_level TEXT NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_ac_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS AC_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme INTEGER NOT NULL,
            high INTEGER NOT NULL,
            moderate INTEGER NOT NULL,
            low INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_saving_throw_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SAVING_THROW_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme INTEGER NOT NULL,
            high INTEGER NOT NULL,
            moderate INTEGER NOT NULL,
            low INTEGER NOT NULL,
            terrible INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_hp_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS HP_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            high_ub INTEGER NOT NULL,
            high_lb INTEGER NOT NULL,
            moderate_ub INTEGER NOT NULL,
            moderate_lb INTEGER NOT NULL,
            low_ub INTEGER NOT NULL,
            low_lb INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_res_weak_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS RES_WEAK_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            max INTEGER NOT NULL,
            min INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_strike_bonus_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS STRIKE_BONUS_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme INTEGER NOT NULL,
            high INTEGER NOT NULL,
            moderate INTEGER NOT NULL,
            low INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_strike_dmg_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS STRIKE_DAMAGE_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            extreme TEXT NOT NULL,
            high TEXT NOT NULL,
            moderate TEXT NOT NULL,
            low TEXT NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_spell_dc_and_atk_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SPELL_DC_AND_ATTACK_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE  NOT NULL,
            extreme_dc INTEGER NOT NULL,
            extreme_atk_bonus INTEGER NOT NULL,
            high_dc INTEGER NOT NULL,
            high_atk_bonus INTEGER NOT NULL,
            moderate_dc INTEGER NOT NULL,
            moderate_atk_bonus INTEGER NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_area_dmg_scales<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS AREA_DAMAGE_SCALES_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level INTEGER UNIQUE NOT NULL,
            unlimited_use TEXT NOT NULL,
            limited_use TEXT NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}
