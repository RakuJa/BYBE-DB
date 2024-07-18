use crate::trait_db_initializer::init_trait_table;
use anyhow::Result;
use sqlx::{Sqlite, Transaction};

pub async fn init_all_item_related_table<'a>(tx: &mut Transaction<'a, Sqlite>) -> Result<()> {
    init_item_table(tx).await?;
    init_trait_table(tx).await?;
    init_rune_table(tx).await?;
    init_weapon_table(tx).await?;
    init_weapon_damage_table(tx).await?;
    init_armor_table(tx).await?;
    init_shield_table(tx).await?;

    init_rune_weapon_association_table(tx).await?;
    init_rune_armor_association_table(tx).await?;

    init_trait_item_association_table(tx).await?;
    init_trait_weapon_association_table(tx).await?;
    init_trait_shield_association_table(tx).await?;
    init_trait_armor_association_table(tx).await?;
    Ok(())
}

pub async fn init_item_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS ITEM_TABLE (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        bulk REAL NOT NULL,
        base_item TEXT,
        category TEXT,
        description TEXT NOT NULL,
        hardness INTEGER NOT NULL,
        hp INTEGER NOT NULL,
        level INTEGER NOT NULL,
        price INTEGER NOT NULL,
        usage TEXT,
        item_group TEXT,
        item_type TEXT NOT NULL,
        material_grade TEXT,
        material_type TEXT,
        number_of_uses INTEGER,

        license TEXT NOT NULL,
        remaster BOOL NOT NULL,
        source TEXT NOT NULL,

        rarity TEXT NOT NULL,
        size TEXT NOT NULL,

        UNIQUE(
            name, bulk, description, hardness, hp, level, price,
            item_type, license, remaster, source, rarity, size
        ) ON CONFLICT ABORT
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

async fn init_trait_item_association_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS TRAIT_ITEM_ASSOCIATION_TABLE (
            item_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (item_id, trait_id),
            FOREIGN KEY (item_id) REFERENCES ITEM_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

async fn init_trait_weapon_association_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS TRAIT_WEAPON_ASSOCIATION_TABLE (
            weapon_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (weapon_id, trait_id),
            FOREIGN KEY (weapon_id) REFERENCES WEAPON_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

async fn init_trait_shield_association_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS TRAIT_SHIELD_ASSOCIATION_TABLE (
            shield_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (shield_id, trait_id),
            FOREIGN KEY (shield_id) REFERENCES SHIELD_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

async fn init_trait_armor_association_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS TRAIT_ARMOR_ASSOCIATION_TABLE (
            armor_id INTEGER NOT NULL,
            trait_id TEXT NOT NULL,
            PRIMARY KEY (armor_id, trait_id),
            FOREIGN KEY (armor_id) REFERENCES ARMOR_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_weapon_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS WEAPON_TABLE (
        id INTEGER PRIMARY KEY NOT NULL,

        to_hit_bonus INTEGER,
        splash_dmg INTEGER,

        n_of_potency_runes INTEGER NOT NULL,
        n_of_striking_runes INTEGER NOT NULL,
        range INTEGER,
        reload TEXT,
        weapon_type TEXT NOT NULL,

        base_item_id INTEGER,
        FOREIGN KEY (base_item_id) REFERENCES ITEM_TABLE(id) ON UPDATE CASCADE
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_weapon_damage_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS WEAPON_DAMAGE_TABLE (
        id INTEGER PRIMARY KEY NOT NULL,

        bonus_dmg INTEGER NOT NULL,
        dmg_type TEXT,
        number_of_dice INTEGER,
        die_size INTEGER,

        weapon_id INTEGER NOT NULL,
        FOREIGN KEY (weapon_id) REFERENCES WEAPON_TABLE(id) ON UPDATE CASCADE
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_armor_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS ARMOR_TABLE (
        id INTEGER PRIMARY KEY NOT NULL,

        bonus_ac INTEGER NOT NULL,
        check_penalty INTEGER NOT NULL,
        dex_cap INTEGER NOT NULL,
        n_of_potency_runes INTEGER NOT NULL,
        n_of_resilient_runes INTEGER NOT NULL,
        speed_penalty INTEGER NOT NULL,
        strength_required INTEGER,

        base_item_id INTEGER,
        FOREIGN KEY (base_item_id) REFERENCES ITEM_TABLE(id) ON UPDATE CASCADE
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_shield_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS SHIELD_TABLE (
        id INTEGER PRIMARY KEY NOT NULL,

        bonus_ac INTEGER NOT NULL,

        n_of_reinforcing_runes INTEGER NOT NULL,

        speed_penalty INTEGER NOT NULL,

        base_item_id INTEGER,
        FOREIGN KEY (base_item_id) REFERENCES ITEM_TABLE(id) ON UPDATE CASCADE
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_rune_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS RUNE_TABLE (
        name TEXT NOT NULL PRIMARY KEY
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_rune_weapon_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS RUNE_WEAPON_ASSOCIATION_TABLE (
            weapon_id INTEGER NOT NULL,
            rune_id TEXT NOT NULL,
            PRIMARY KEY (weapon_id, rune_id),
            FOREIGN KEY (weapon_id) REFERENCES WEAPON_TABLE(id),
            FOREIGN KEY (rune_id) REFERENCES RUNE_TABLE(name)
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}

pub async fn init_rune_armor_association_table<'a>(
    conn: &mut Transaction<'a, Sqlite>,
) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS RUNE_ARMOR_ASSOCIATION_TABLE (
            armor_id INTEGER NOT NULL,
            rune_id TEXT NOT NULL,
            PRIMARY KEY (armor_id, rune_id),
            FOREIGN KEY (armor_id) REFERENCES ARMOR_TABLE(id),
            FOREIGN KEY (rune_id) REFERENCES RUNE_TABLE(name)
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}
