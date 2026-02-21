mod db;
mod schema;
mod utils;

extern crate core;
extern crate dotenvy;
extern crate git2;

use crate::db::db_handler_one;
use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeItemParsingError, BybeShield, BybeWeapon};
use crate::schema::source_schema::creature::source_creature::{
    SourceCreature, SourceCreatureParsingError,
};
use crate::utils::game_system_enum::GameSystem;
use crate::utils::json_manual_fetcher::get_json_paths;

use dotenvy::dotenv;
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::{backtrace, env, fs};
use tracing::warn;
use tracing::{debug, error};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    dotenv().ok(); // use dotenv env variables
    let file_appender = rolling::daily("./logs", "app.log");
    let (file_writer, _guard) = non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::new("info")) // or "debug", "warn", "myapp=debug,hyper=warn"
        .with(fmt::layer().with_writer(file_writer))
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();
    let pf2e_source_url = &env::var("PF2E_SOURCE_URL")
        .or_else(|_| env::var("SOURCE_URL"))
        .expect("PF2E SOURCE URL NOT SET.. Aborting. Hint: set SOURCE_URL environmental variable");

    let sf2e_source_url = &env::var("SF2E_SOURCE_URL")
        .expect("SF2E SOURCE URL NOT SET.. Aborting. Hint: set SOURCE_URL environmental variable");

    let source_path = &env::var("SOURCE_DOWNLOAD_PATH").expect(
        "DOWNLOAD PATH NOT SET.. Aborting. Hint: set SOURCE_DOWNLOAD_PATH environmental variable",
    );
    let db_url = &env::var("DATABASE_URL")
        .expect("DB URL IS NOT SET.. Aborting. Hint: set DATABASE_URL environmental variable");
    let conn = db_handler_one::connect(db_url)
        .await
        .expect("Could not connect to the given db url, something went wrong..");

    let pf2e_source_path = format!("{}/{}/", source_path, GameSystem::Pathfinder);
    let sf2e_source_path = format!("{}/{}/", source_path, GameSystem::Starfinder);
    fs::create_dir_all(pf2e_source_path.as_str())
        .expect("Could not create folder to store PF2E raw data");
    fs::create_dir_all(sf2e_source_path.as_str())
        .expect("Could not create folder to store SF2E raw data");

    fetch_source_data(pf2e_source_url, pf2e_source_path.as_str());
    fetch_source_data(sf2e_source_url, sf2e_source_path.as_str());

    let pf2e_json_paths = get_json_paths(pf2e_source_path.as_str());
    let sf2e_json_paths = get_json_paths(sf2e_source_path.as_str());

    clear_db(&conn).await.unwrap();

    db_update(&conn, pf2e_json_paths, sf2e_json_paths)
        .await
        .unwrap();
}

async fn clear_db(conn: &SqlitePool) -> anyhow::Result<()> {
    db_handler_one::drop_tables_except(
        conn,
        &[
            "pf_item_table",
            "pf_creature_table",
            "sf_item_table",
            "sf_creature_table",
        ],
    )
    .await?;
    Ok(())
}

async fn db_update(
    conn: &SqlitePool,
    pf2e_json_paths: Vec<String>,
    sf2e_json_paths: Vec<String>,
) -> anyhow::Result<()> {
    let mut tx: Transaction<Sqlite> = conn.begin().await?;

    game_system_tables_update(&mut tx, pf2e_json_paths, &GameSystem::Pathfinder).await?;

    db_handler_one::update_with_aon_data(&mut tx).await?;

    game_system_tables_update(&mut tx, sf2e_json_paths, &GameSystem::Starfinder).await?;

    db_handler_one::insert_scales_values_to_db(&mut tx).await?;

    tx.commit().await?;
    Ok(())
}

async fn game_system_tables_update(
    tx: &mut Transaction<'_, Sqlite>,
    json_paths: Vec<String>,
    gs: &GameSystem,
) -> anyhow::Result<()> {
    for el in deserialize_json_items(&json_paths) {
        if let Err(e) = db_handler_one::insert_item_to_db(tx, gs, &el, None).await {
            error!(
                "Failed to insert item: {:?}, skipping with error {:?}",
                el, e
            );
        }
    }
    for el in deserialize_json_armors(&json_paths) {
        if let Err(e) = db_handler_one::insert_armor_to_db(tx, gs, &el, None).await {
            error!(
                "Failed to insert armor: {:?}, skipping with error {:?}",
                el, e
            );
        }
    }
    for el in deserialize_json_shields(&json_paths) {
        if let Err(e) = db_handler_one::insert_shield_to_db(tx, gs, &el, None).await {
            error!(
                "Failed to insert shield: {:?}, skipping with error {:?}",
                el, e
            );
        }
    }
    for el in deserialize_json_weapons(&json_paths) {
        if let Err(e) = db_handler_one::insert_weapon_to_db(tx, gs, &el, None).await {
            error!(
                "Failed to insert weapon: {:?}, skipping with error {:?}",
                el, e
            );
        }
    }
    // we add creature as last. This is made to avoid useless duplicates for
    // item, weapons, etc
    for el in deserialize_json_creatures(&json_paths) {
        if let Err(e) = db_handler_one::insert_creature_to_db(tx, gs, &el).await {
            error!(
                "Failed to insert creature: {:?}, skipping with error {:?}",
                el, e
            );
        }
    }
    Ok(())
}

fn deserialize_json_creatures(json_files: &Vec<String>) -> Vec<BybeCreature> {
    let mut creatures = Vec::new();
    for file in json_files {
        match SourceCreature::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            Ok(creature) => creatures.push(BybeCreature::from(creature)),
            Err(e) => match e {
                SourceCreatureParsingError::DuplicatedCreature
                | SourceCreatureParsingError::InvalidCreatureType => {}
                _ => panic!(
                    "Error parsing creature {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    creatures
}

fn deserialize_json_items(json_files: &Vec<String>) -> Vec<BybeItem> {
    let mut items = Vec::new();
    for file in json_files {
        match BybeItem::try_from((
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
            false,
        )) {
            Ok(item) => items.push(item),
            Err(e) => match e {
                BybeItemParsingError::InvalidItemType
                | BybeItemParsingError::UnsupportedItemType => {}
                _ => panic!(
                    "Error parsing weapon {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    items
}

fn deserialize_json_weapons(json_files: &Vec<String>) -> Vec<BybeWeapon> {
    let mut weapons = Vec::new();
    for file in json_files {
        match BybeWeapon::try_from((
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
            false,
        )) {
            Ok(item) => weapons.push(item),
            Err(e) => match e {
                BybeItemParsingError::InvalidItemType
                | BybeItemParsingError::UnsupportedItemType => {}
                _ => panic!(
                    "Error parsing weapon {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    weapons
}

fn deserialize_json_armors(json_files: &Vec<String>) -> Vec<BybeArmor> {
    let mut armors = Vec::new();
    for file in json_files {
        match BybeArmor::try_from((
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
            false,
        )) {
            Ok(item) => armors.push(item),
            Err(e) => match e {
                BybeItemParsingError::InvalidItemType
                | BybeItemParsingError::UnsupportedItemType => {}
                _ => panic!(
                    "Error parsing weapon {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    armors
}

fn deserialize_json_shields(json_files: &Vec<String>) -> Vec<BybeShield> {
    let mut shields = Vec::new();
    for file in json_files {
        match BybeShield::try_from((
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
            false,
        )) {
            Ok(item) => shields.push(item),
            Err(e) => match e {
                BybeItemParsingError::InvalidItemType
                | BybeItemParsingError::UnsupportedItemType => {}
                _ => panic!(
                    "Error parsing weapon {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    shields
}

fn is_dir_empty(path: &str) -> bool {
    fs::read_dir(path)
        .map(|mut e| e.next().is_none())
        .unwrap_or(false)
}

fn fetch_source_data(source_url: &str, source_path: &str) {
    // Clones source if the given path is empty, otherwise warns
    // But keeps executing
    if is_dir_empty(source_path) {
        debug!("Cloning path: {source_path}");
        match git2::build::RepoBuilder::new()
            //.branch("6.8.5")
            .clone(source_url, source_path.as_ref())
        {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {e}"),
        };
    } else {
        warn!("Path already exists, won't clone source dataset.")
    }
}

fn read_from_file_to_string(creature_file: &str) -> String {
    fs::read_to_string(creature_file)
        .unwrap_or_else(|_| panic!("Unable to read file {creature_file}"))
}
