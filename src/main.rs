mod db;
mod schema;
mod utils;

extern crate dotenvy;
extern crate git2;

use crate::db::db_handler_one;
use crate::schema::bybe_creature::BybeCreature;
use crate::schema::source_schema::creature::source_creature::SourceCreature;
use dotenvy::dotenv;
use git2::Repository;
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
    let conn = db::db_handler_one::connect(db_url)
        .await
        .expect("Could not connect to the given db url, something went wrong..");
    fetch_source_data(source_url, source_path);
    let x = deserialize_json_bestiaries(get_creature_json_paths(get_bestiary_paths(source_path)));
    for el in x {
        // add db cleanup if result is failure
        let bb_creature = BybeCreature::init_from_source_creature(el);
        db_handler_one::insert_creature_to_db(&conn, bb_creature.clone())
            .await
            .expect("Something failed while inserting creature in db");
    }
    db_handler_one::insert_scales_values_to_db(&conn)
        .await
        .expect("Something failed while insert scale values in db");
    //println!("{:?}", x);
}

fn deserialize_json_bestiaries(json_creatures_list: Vec<String>) -> Vec<SourceCreature> {
    let mut creature_list = vec![];
    for creature_file in json_creatures_list {
        let x = SourceCreature::init_from_json(
            serde_json::from_str(&read_from_file_to_string(creature_file.as_str()))
                .expect("JSON was not well-formatted"),
        );
        if let Some(creature) = x {
            creature_list.push(creature);
        }
    }
    creature_list
}

fn get_creature_json_paths(bestiary_list: Vec<String>) -> Vec<String> {
    let mut json_list = vec![];
    for bestiary in bestiary_list {
        let paths = fs::read_dir(bestiary.clone())
            .unwrap_or_else(|_| panic!("Could not open bestiary named: {}", bestiary));
        for path in paths {
            match path {
                Ok(cr_json_path) => {
                    let x = cr_json_path.path().to_str().unwrap_or_default().to_string();
                    if !x.is_empty() && x.ends_with(".json") {
                        json_list.push(x)
                    }
                }
                Err(err) => println!("Skipping path with error: {}", err),
            }
        }
    }
    json_list
}

fn get_bestiary_paths(source_path: &str) -> Vec<String> {
    let bestiaries_path = format!("{}/packs", source_path);
    let paths =
        fs::read_dir(bestiaries_path).expect("No valid packs folder found in source dataset");
    let mut bestiary_folders = vec![];
    for path in paths {
        match path {
            Ok(dir_path) => {
                let path = dir_path.path().to_str().unwrap_or_default().to_string();
                bestiary_folders.push(path);
            }
            Err(err) => println!("Skipping path with error: {}", err),
        };
    }
    bestiary_folders
}
fn read_from_file_to_string(creature_file: &str) -> String {
    fs::read_to_string(creature_file)
        .unwrap_or_else(|_| panic!("Unable to read file {}", creature_file))
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
