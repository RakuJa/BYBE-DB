use sqlx::{Sqlite, Transaction};

pub async fn init_all_creature_related_tables<'a>(
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
    init_action_table(tx).await?;
    init_trait_action_association_table(tx).await?;
    init_skill_table(tx).await?;
    init_skill_modifier_variant_table(tx).await?;
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

            spell_casting_name TEXT,
            is_spell_casting_flexible BOOL,
            type_of_spell_caster TEXT,
            spell_casting_dc_mod TEXT,
            spell_casting_atk_mod INTEGER,
            spell_casting_tradition TEXT
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
// END SPELL
// BEGIN ACTION

async fn init_action_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS ACTION_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            action_type TEXT NOT NULL,
            n_of_actions INTEGER,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            license TEXT NOT NULL,
            remaster BOOL NOT NULL,
            source TEXT NOT NULL,
            slug TEXT,
            rarity TEXT NOT NULL,
            creature_id INTEGER NOT NULL,
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    )
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_trait_action_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS TRAIT_ACTION_ASSOCIATION_TABLE (
            action_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (action_id, trait_id),
            FOREIGN KEY (action_id) REFERENCES ACTION_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

// END ACTION
// BEGIN SKILL

async fn init_skill_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS SKILL_TABLE (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            modifier INTEGER NOT NULL,
            proficiency INTEGER NOT NULL,
            license TEXT NOT NULL,
            remaster BOOL NOT NULL,
            source TEXT NOT NULL,
            creature_id INTEGER NOT NULL,
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
    )
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}

async fn init_skill_modifier_variant_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> anyhow::Result<bool> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS CREATURE_SKILL_LABEL_TABLE (
            creature_id INTEGER NOT NULL,
            skill_id INTEGER NOT NULL,
            skill_label TEXT NOT NULL,
            PRIMARY KEY (creature_id, skill_id, skill_label),
            FOREIGN KEY (creature_id) REFERENCES CREATURE_TABLE(id)
            FOREIGN KEY (skill_id) REFERENCES SKILL_TABLE(id)
    );
    "#;
    sqlx::query(query).execute(&mut **conn).await?;
    Ok(true)
}
