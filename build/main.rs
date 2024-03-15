mod creature_db_initializer;
mod scales_db_initializer;

use crate::creature_db_initializer::init_all_creature_related_tables;
use crate::scales_db_initializer::init_creature_builder_tables;
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
