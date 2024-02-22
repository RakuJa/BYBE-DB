mod db;
mod schema;

extern crate git2;

use crate::schema::bybe_creature::BybeCreature;
use crate::schema::foundry_schema::creature::foundry_creature::FoundryCreature;
use git2::Repository;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    let foundry_url = "https://github.com/foundryvtt/pf2e";
    let foundry_path = "./data/foundry";
    let db_path = "./db/database.db";
    fetch_foundry_data(foundry_url, foundry_path);
    let x = deserialize_json_bestiaries(get_creature_json_paths(get_bestiary_paths(foundry_path)));
    for el in x {
        BybeCreature::init_from_foundry_creature(el);
    }
    //println!("{:?}", x);
    fs::create_dir_all("./db/").expect("TODO: panic message");
    let conn = db::db_handler_one::connect(db_path)
        .await
        .expect("TODO: panic message");
    db::db_handler_one::init_tables(&conn)
        .await
        .expect("TODO: panic message");
}

fn deserialize_json_bestiaries(json_creatures_list: Vec<String>) -> Vec<FoundryCreature> {
    let mut creature_list = vec![];
    for creature_file in json_creatures_list {
        let x = FoundryCreature::init_from_json(
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

fn get_bestiary_paths(foundry_path: &str) -> Vec<String> {
    let bestiaries_path = format!("{}/packs", foundry_path);
    let paths =
        fs::read_dir(bestiaries_path).expect("No valid packs folder found in foundry dataset");
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

fn fetch_foundry_data(foundry_url: &str, foundry_path: &str) {
    // Clones foundry if the given path is empty, otherwise warns
    // But keeps executing
    if !Path::new(foundry_path).exists() {
        match Repository::clone(foundry_url, foundry_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    } else {
        println!("Warning: Path already exists, won't clone foundry dataset.")
    }
}
