mod db;
mod schema;
mod utils;

extern crate core;
extern crate dotenvy;
extern crate git2;

use crate::db::db_handler_one;
use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_item::{BybeArmor, BybeItem, BybeWeapon};
use crate::schema::source_schema::creature::source_creature::SourceCreature;
use crate::utils::json_manual_fetcher::get_json_paths;
use dotenvy::dotenv;
use git2::Repository;
use sqlx::{Sqlite, Transaction};
use std::path::Path;
use std::{env, fs};

#[tokio::main]
async fn main() {
    dotenv().ok(); // use dotenv env variables
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

    for el in deserialize_json_creatures(&json_paths) {
        db_handler_one::insert_creature_to_db(&conn, el)
            .await
            .expect("Something failed while inserting creature in db");
    }
    for el in deserialize_json_items(&json_paths) {
        db_handler_one::insert_item_to_db(&conn, el)
            .await
            .expect("Something failed while inserting item in db");
    }
    let mut tx: Transaction<Sqlite> = conn.begin().await.unwrap();
    for el in deserialize_json_armors(&json_paths) {
        db_handler_one::insert_armor_to_db(&mut tx, &el)
            .await
            .expect("Something failed while inserting armor in db");
    }

    for el in deserialize_json_weapons(&json_paths) {
        db_handler_one::insert_weapon_to_db(&mut tx, &el)
            .await
            .expect("Something failed while inserting weapon in db");
    }
    tx.commit().await.unwrap();
    db_handler_one::insert_scales_values_to_db(&conn)
        .await
        .expect("Something failed while insert scale values in db");

    db_handler_one::update_with_aon_data(&conn)
        .await
        .expect("Could not update Database using AON data");
}

fn deserialize_json_creatures(json_files: &Vec<String>) -> Vec<BybeCreature> {
    let mut creatures = Vec::new();
    for file in json_files {
        if let Some(creature) = SourceCreature::init_from_json(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            creatures.push(BybeCreature::init_from_source_creature(creature));
        }
    }
    creatures
}

fn deserialize_json_items(json_files: &Vec<String>) -> Vec<BybeItem> {
    let mut items = Vec::new();
    for file in json_files {
        if let Some(item) = BybeItem::init_from_json(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            items.push(item);
        }
    }
    items
}

fn deserialize_json_weapons(json_files: &Vec<String>) -> Vec<BybeWeapon> {
    let mut weapons = Vec::new();
    for file in json_files {
        if let Some(item) = BybeWeapon::init_from_json(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            weapons.push(item);
        }
    }
    weapons
}

fn deserialize_json_armors(json_files: &Vec<String>) -> Vec<BybeArmor> {
    let mut armors = Vec::new();
    for file in json_files {
        if let Some(item) = BybeArmor::init_from_json(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            armors.push(item);
        }
    }
    armors
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
        println!("Warning: Path already exists, won't clone source dataset.")
    }
}

fn read_from_file_to_string(creature_file: &str) -> String {
    fs::read_to_string(creature_file)
        .unwrap_or_else(|_| panic!("Unable to read file {}", creature_file))
}
