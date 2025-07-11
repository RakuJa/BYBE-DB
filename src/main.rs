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
use crate::utils::json_manual_fetcher::get_json_paths;
use dotenvy::dotenv;
use git2::Repository;
use log::warn;
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::path::Path;
use std::{backtrace, env, fs};

#[tokio::main]
async fn main() {
    dotenv().ok(); // use dotenv env variables
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));
    let source_url = &env::var("SOURCE_URL")
        .expect("SOURCE URL NOT SET.. Aborting. Hint: set SOURCE_URL environmental variable");
    let source_path = &env::var("SOURCE_DOWNLOAD_PATH").expect(
        "DOWNLOAD PATH NOT SET.. Aborting. Hint: set SOURCE_DOWNLOAD_PATH environmental variable",
    );
    let db_url = &env::var("DATABASE_URL")
        .expect("DB URL IS NOT SET.. Aborting. Hint: set DATABASE_URL environmental variable");
    let conn = db_handler_one::connect(db_url)
        .await
        .expect("Could not connect to the given db url, something went wrong..");
    fetch_source_data(source_url, source_path);

    let json_paths = get_json_paths(source_path);
    db_update(&conn, json_paths).await.unwrap();
}

async fn db_update(conn: &SqlitePool, json_paths: Vec<String>) -> anyhow::Result<()> {
    let mut tx: Transaction<Sqlite> = conn.begin().await?;
    for el in deserialize_json_items(&json_paths) {
        db_handler_one::insert_item_to_db(&mut tx, &el, None).await?;
    }
    for el in deserialize_json_armors(&json_paths) {
        db_handler_one::insert_armor_to_db(&mut tx, &el, None).await?;
    }
    for el in deserialize_json_shields(&json_paths) {
        db_handler_one::insert_shield_to_db(&mut tx, &el, None).await?;
    }
    for el in deserialize_json_weapons(&json_paths) {
        db_handler_one::insert_weapon_to_db(&mut tx, &el, None).await?;
    }
    // we add creature as last. This is made to avoid useless duplicates for
    // item, weapons, etc
    for el in deserialize_json_creatures(&json_paths) {
        db_handler_one::insert_creature_to_db(&mut tx, el).await?;
    }
    db_handler_one::insert_scales_values_to_db(&mut tx).await?;

    db_handler_one::update_with_aon_data(&mut tx).await?;
    tx.commit().await?;
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
        match BybeItem::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
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
        match BybeWeapon::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
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
        match BybeArmor::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
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
        match BybeShield::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
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

fn fetch_source_data(source_url: &str, source_path: &str) {
    // Clones source if the given path is empty, otherwise warns
    // But keeps executing
    if !Path::new(source_path).exists() {
        match Repository::clone(source_url, source_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    } else {
        warn!("Path already exists, won't clone source dataset.")
    }
}

fn read_from_file_to_string(creature_file: &str) -> String {
    fs::read_to_string(creature_file)
        .unwrap_or_else(|_| panic!("Unable to read file {}", creature_file))
}
