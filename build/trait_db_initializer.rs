use sqlx::{Sqlite, Transaction};

pub async fn init_trait_table<'a>(conn: &mut Transaction<'a, Sqlite>) -> anyhow::Result<bool> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS TRAIT_TABLE (
            name TEXT PRIMARY KEY NOT NULL
    );
    ",
    )
    .execute(&mut **conn)
    .await?;
    Ok(true)
}
