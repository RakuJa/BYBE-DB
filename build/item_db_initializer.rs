use crate::trait_db_initializer::init_trait_table;
use anyhow::Result;
use sqlx::{Sqlite, Transaction};

pub async fn init_all_item_related_table<'a>(tx: &mut Transaction<'a, Sqlite>) -> Result<()> {
    init_item_tables(tx).await?;
    init_trait_table(tx).await?;
    init_trait_item_association_table(tx).await?;
    Ok(())
}

pub async fn init_item_tables<'a>(conn: &mut Transaction<'a, Sqlite>) -> Result<()> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS ITEM_TABLE (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        bulk REAL NOT NULL,
        category TEXT,
        description TEXT NOT NULL,
        hardness INTEGER NOT NULL,
        hp INTEGER NOT NULL,
        level INTEGER NOT NULL,
        price INTEGER NOT NULL,
        usage TEXT NOT NULL,
        item_type TEXT NOT NULL,
        material_grade TEXT,
        material_type TEXT,
        number_of_uses INTEGER,

        license TEXT NOT NULL,
        remaster BOOL NOT NULL,
        source TEXT NOT NULL,

        rarity TEXT NOT NULL,
        size TEXT NOT NULL
    );
    "
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
            FOREIGN KEY (item_id) REFERENCES SPELL_TABLE(id),
            FOREIGN KEY (trait_id) REFERENCES TRAIT_TABLE(name)
    );
    "
    )
    .execute(&mut **conn)
    .await?;
    Ok(())
}
