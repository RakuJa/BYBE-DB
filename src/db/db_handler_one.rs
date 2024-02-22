use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::path::Path;

pub async fn connect(filename: impl AsRef<Path>) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    Ok(SqlitePool::connect_with(options).await?)
}

pub async fn init_tables(conn: &SqlitePool) -> Result<bool> {
    Ok(init_creature_table(conn).await?
        && init_trait_table(conn).await?
        && init_trait_cr_association_table(conn).await?
        && init_language_table(conn).await?
        && init_language_cr_association_table(conn).await?
        && init_immunity_table(conn).await?
        && init_immunity_cr_association_table(conn).await?
        && init_sense_table(conn).await?
        && init_sense_cr_association_table(conn).await?
        && init_speed_table(conn).await?
        && init_resistances_table(conn).await?
        && init_weakness_table(conn).await?
        && init_weapon_table(conn).await?
        && init_trait_weapon_association_table(conn).await?
        && init_spell_table(conn).await?
        && init_trait_spell_association_table(conn).await?
        && init_tradition_table(conn).await?
        && init_tradition_spell_association_table(conn).await?)
}

async fn init_creature_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS CREATURE_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            aon_id INTEGER,
            charisma INTEGER NOT NULL,
            constitution INTEGER NOT NULL,
            dexterity INTEGER NOT NULL,
            intelligence INTEGER NOT NULL,
            strength INTEGER NOT NULL,
            wisdom INTEGER NOT NULL,
            ac INTEGER NOT NULL,
            hp INTEGER NOT NULL,
            hp_details TEXT NOT NULL,
            ac_details TEXT NOT NULL,
            language_detail TEXT,
            level INTEGER NOT NULL,
            license VARCHAR NOT NULL,
            remaster BOOL NOT NULL,
            source TEXT NOT NULL,
            initiative_ability TEXT NOT NULL,
            perception INTEGER NOT NULL,
            perception_details VARCHAR NOT NULL,
            fortitude INTEGER NOT NULL,
            reflex INTEGER NOT NULL,
            will INTEGER NOT NULL,
            fortitude_detail TEXT NOT NULL,
            reflex_detail TEXT NOT NULL,
            will_detail TEXT NOT NULL,
            rarity TEXT NOT NULL,
            size TEXT NOT NULL,


            type TEXT,

            is_ranged BOOL NOT NULL,
            is_melee BOOL NOT NULL,
            is_spell_caster BOOL NOT NULL
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_trait_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_TABLE (
            name TEXT PRIMARY KEY
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_trait_cr_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, trait_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_speed_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SPEED_TABLE (
            creature_id INTEGER NOT NULL,
            type TEXT NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (creature_id, type),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_resistances_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS RESISTANCE_TABLE (
            creature_id INTEGER NOT NULL,
            type TEXT NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (creature_id, type),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_weakness_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS WEAKNESS_TABLE (
            creature_id INTEGER NOT NULL,
            type TEXT NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (creature_id, type),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_immunity_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS IMMUNITY_TABLE (
            name TEXT PRIMARY KEY
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_immunity_cr_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS IMMUNITY_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            immunity_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, immunity_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (immunity_id) REFERENCES IMMUNITY_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_language_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS LANGUAGE_TABLE (
            name TEXT PRIMARY KEY
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_language_cr_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS LANGUAGE_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            language_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, language_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (language_id) REFERENCES LANGUAGE_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_sense_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SENSE_TABLE (
            name TEXT PRIMARY KEY
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_sense_cr_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SENSE_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            sense_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, sense_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (sense_id) REFERENCES SENSE_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_weapon_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS WEAPON_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            base TEXT NOT NULL,
            to_hit_bonus INTEGER NOT NULL,
            bulk INTEGER NOT NULL,
            category TEXT NOT NULL,
            dmg_type TEXT,
            n_of_dices INTEGER,
            die_size TEXT,
            bonus_dmg INTEGER,
            carry_type TEXT,
            hands_held INTEGER,
            invested BOOL,
            weapon_group TEXT NOT NULL,
            hardness INTEGER,
            hp_max INTEGER,
            hp_curr INTEGER,
            level INTEGER,
            license TEXT NOT NULL,
            remastered BOOL NOT NULL,
            source TEXT NOT NULL,
            quantity INTEGER,
            range TEXT,
            reload TEXT,
            size TEXT NOT NULL,
            rarity TEXT NOT NULL,
            usage TEXT NOT NULL,
            type TEXT NOT NULL
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_trait_weapon_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_WEAPON_ASSOCIATION_TABLE (
            weapon_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (weapon_id, trait_id),
            FOREIGN KEY (weapon_id) REFERENCES WEAPON_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_spell_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SPELL_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            area_type TEXT,
            area_value INTEGER,
            counteraction BOOL NOT NULL,

            saving_throw_is_basic BOOL,
            saving_throw_statistic TEXT,

            sustained BOOL NOT NULL,
            duration TEXT,
            level INTEGER NOT NULL,
            range TEXT NOT NULL,
            target TEXT NOT NULL,
            action TEXT NOT NULL,
            license TEXT NOT NULL,
            remastered BOOL NOT NULL,
            source TEXT NOT NULL,
            rarity TEXT NOT NULL
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_trait_spell_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_SPELL_ASSOCIATION_TABLE (
            spell_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (spell_id, trait_id),
            FOREIGN KEY (spell_id) REFERENCES SPELL_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_tradition_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRADITION_TABLE (
            name TEXT PRIMARY KEY
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}

async fn init_tradition_spell_association_table(conn: &SqlitePool) -> Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRADITION_SPELL_ASSOCIATION_TABLE (
            spell_id INTEGER NOT NULL,
            tradition_id TEXT NOT NULL,
            PRIMARY KEY (spell_id, tradition_id),
            FOREIGN KEY (spell_id) REFERENCES SPELL_TABLE(id),
            FOREIGN KEY (tradition_id) REFERENCES TRADITION_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(conn).await?;
    Ok(true)
}
