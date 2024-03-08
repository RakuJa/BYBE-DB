use dotenv::dotenv;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::str::FromStr;
use std::{env, fs};

#[tokio::main]
async fn main() {
    dotenv().ok(); // use dotenv env variables
    let db_url = &env::var("DATABASE_URL")
        .expect("DB URL IS NOT SET.. Aborting. Hint: set DATABASE_URL environmental variable");
    let db_path = &env::var("DATABASE_PATH")
        .expect("DB PATH IS NOT SET.. Aborting. Hint: set DATABASE_PATH environmental variable");

    fs::create_dir_all(db_path).expect("Could not create parent folder to save db.");

    let conn = SqlitePool::connect_with(
        SqliteConnectOptions::from_str(db_url)
            .expect("Could not find a valid db in the given path")
            .create_if_missing(true),
    )
    .await
    .expect("Could not connect to the given db url, something went wrong..");
    init_tables(&conn)
        .await
        .expect("Could not initialize tables inside the db, something went wrong..");
}

pub async fn init_tables(conn: &SqlitePool) -> anyhow::Result<bool> {
    let mut tx: Transaction<Sqlite> = conn.begin().await?;
    init_all_creature_related_tables(&mut tx).await?;
    init_creature_builder_tables(&mut tx).await?;
    tx.commit().await?;
    Ok(true)
}

async fn init_all_creature_related_tables<'a>(
    tx: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    init_creature_table(tx).await?;
    init_trait_table(tx).await?;
    init_trait_cr_association_table(tx).await?;
    init_language_table(tx).await?;
    init_language_cr_association_table(tx).await?;
    init_immunity_table(tx).await?;
    init_immunity_cr_association_table(tx).await?;
    init_sense_table(tx).await?;
    init_sense_cr_association_table(tx).await?;
    init_speed_table(tx).await?;
    init_resistances_table(tx).await?;
    init_weakness_table(tx).await?;
    init_weapon_table(tx).await?;
    init_trait_weapon_association_table(tx).await?;
    init_spell_table(tx).await?;
    init_trait_spell_association_table(tx).await?;
    init_tradition_table(tx).await?;
    init_tradition_spell_association_table(tx).await?;
    Ok(true)
}

async fn init_creature_builder_tables<'a>(
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

async fn init_creature_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
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
            hp_detail TEXT NOT NULL,
            ac_detail TEXT NOT NULL,
            language_detail TEXT,
            level INTEGER NOT NULL,
            license TEXT NOT NULL,
            remaster BOOL NOT NULL,
            source TEXT NOT NULL,
            initiative_ability TEXT NOT NULL,
            perception INTEGER NOT NULL,
            perception_detail TEXT NOT NULL,
            fortitude INTEGER NOT NULL,
            reflex INTEGER NOT NULL,
            will INTEGER NOT NULL,
            fortitude_detail TEXT NOT NULL,
            reflex_detail TEXT NOT NULL,
            will_detail TEXT NOT NULL,
            rarity TEXT NOT NULL,
            size TEXT NOT NULL,


            cr_type TEXT,
            family TEXT,

            is_spell_caster BOOL NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_trait_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS TRAIT_TABLE (
            name TEXT PRIMARY KEY NOT NULL
    );
    "#,
    )
    .execute(&mut **conn)
    .await?;
    Ok(true)
}

async fn init_trait_cr_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, trait_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_speed_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SPEED_TABLE (
            creature_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (creature_id, name),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_resistances_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS RESISTANCE_TABLE (
            creature_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (creature_id, name),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_weakness_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS WEAKNESS_TABLE (
            creature_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (creature_id, name),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_immunity_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS IMMUNITY_TABLE (
            name TEXT PRIMARY KEY NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_immunity_cr_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS IMMUNITY_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            immunity_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, immunity_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (immunity_id) REFERENCES IMMUNITY_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_language_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS LANGUAGE_TABLE (
            name TEXT PRIMARY KEY NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_language_cr_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS LANGUAGE_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            language_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, language_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (language_id) REFERENCES LANGUAGE_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_sense_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SENSE_TABLE (
            name TEXT PRIMARY KEY NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_sense_cr_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SENSE_CREATURE_ASSOCIATION_TABLE (
            creature_id INTEGER NOT NULL,
            sense_id TEXT NOT NULL,
            PRIMARY KEY (creature_id, sense_id),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id),
            FOREIGN KEY (sense_id) REFERENCES SENSE_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_weapon_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
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
            remaster BOOL NOT NULL,
            source TEXT NOT NULL,
            quantity INTEGER,
            range TEXT,
            reload TEXT,
            size TEXT NOT NULL,
            rarity TEXT NOT NULL,
            usage TEXT NOT NULL,
            wp_type TEXT NOT NULL,
            creature_id INTEGER NOT NULL,
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;

    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_trait_weapon_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_WEAPON_ASSOCIATION_TABLE (
            weapon_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (weapon_id, trait_id),
            FOREIGN KEY (weapon_id) REFERENCES WEAPON_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_spell_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
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
            remaster BOOL NOT NULL,
            source TEXT NOT NULL,
            rarity TEXT NOT NULL,
            creature_id INTEGER NOT NULL,
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_trait_spell_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_SPELL_ASSOCIATION_TABLE (
            spell_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (spell_id, trait_id),
            FOREIGN KEY (spell_id) REFERENCES SPELL_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_tradition_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRADITION_TABLE (
            name TEXT PRIMARY KEY NOT NULL
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_tradition_spell_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRADITION_SPELL_ASSOCIATION_TABLE (
            spell_id INTEGER NOT NULL,
            tradition_id TEXT NOT NULL,
            PRIMARY KEY (spell_id, tradition_id),
            FOREIGN KEY (spell_id) REFERENCES SPELL_TABLE(id),
            FOREIGN KEY (tradition_id) REFERENCES TRADITION_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
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
