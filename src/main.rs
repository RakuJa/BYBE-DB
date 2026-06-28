#[cfg(not(feature = "dry-run"))]
mod db;
mod game_system_handler;
mod schema;
mod utils;

extern crate core;
extern crate dotenvy;
extern crate git2;

use crate::schema::bybe_creature::BybeCreature;
use crate::schema::bybe_item::{
    BybeArmor, BybeItem, BybeItemParsingError, BybeShield, BybeWeapon, SourceWeapon,
};
use crate::schema::source_schema::creature::source_creature::{
    SourceCreature, SourceCreatureParsingError,
};
#[cfg(not(feature = "dry-run"))]
use crate::utils::game_system_enum::GameSystem;
use crate::utils::json_manual_fetcher::get_json_paths;

use crate::schema::bybe_hazard::BybeHazard;
use crate::schema::source_schema::hazard::source_hazard::{SourceHazard, SourceHazardParsingError};
use dotenvy::dotenv;
use std::{backtrace, env, fs};
#[cfg(not(feature = "dry-run"))]
use tracing::error;
use tracing::{debug, warn};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

#[cfg(not(feature = "dry-run"))]
use crate::db::db_handler_one;
use crate::game_system_handler::set_game_system;
use crate::schema::bybe_condition::{BybeCondition, BybeConditionParsingError};
#[cfg(not(feature = "dry-run"))]
use sqlx::{PgPool, Postgres, Transaction};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let file_appender = rolling::daily("./logs", "app.log");
    let (file_writer, _guard) = non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(file_writer)
                .with_filter(EnvFilter::new("info")),
        )
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(EnvFilter::new("error")),
        )
        .init();

    let source_url = &env::var("SOURCE_URL")
        .or_else(|_| env::var("SOURCE_URL"))
        .expect("SOURCE URL NOT SET.. Aborting. Hint: set SOURCE_URL environmental variable");

    let source_path = &env::var("SOURCE_DOWNLOAD_PATH").expect(
        "DOWNLOAD PATH NOT SET.. Aborting. Hint: set SOURCE_DOWNLOAD_PATH environmental variable",
    );
    fs::create_dir_all(source_path.as_str())
        .expect("Could not create folder to store source raw data");

    let pf2e_source_path = format!("{}/packs/pf2e/", source_path);
    let sf2e_source_path = format!("{}/packs/sf2e/", source_path);

    fetch_source_data(source_url, source_path.as_str());

    let pf2e_json_paths = get_json_paths(pf2e_source_path.as_str());
    let sf2e_json_paths = get_json_paths(sf2e_source_path.as_str());

    #[cfg(feature = "dry-run")]
    {
        dry_run_check_descriptions(&pf2e_json_paths, "pf2e");
        dry_run_check_descriptions(&sf2e_json_paths, "sf2e");
    }

    #[cfg(not(feature = "dry-run"))]
    {
        let db_url = &env::var("DATABASE_URL")
            .expect("DB URL IS NOT SET.. Aborting. Hint: set DATABASE_URL environmental variable");
        let conn = db_handler_one::connect(db_url)
            .await
            .expect("Could not connect to the given db url, something went wrong..");

        sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
            .await
            .expect("Could not find migrations directory")
            .run(&conn)
            .await
            .expect("Failed to run migrations");

        clear_db(&conn).await.unwrap();
        db_update(&conn, pf2e_json_paths, sf2e_json_paths)
            .await
            .unwrap();
    }
}

#[cfg(feature = "dry-run")]
fn dry_run_check_descriptions(json_paths: &[String], system: &str) {
    use crate::utils::tag::tag_parser::find_remaining_tags;

    let mut total_issues = 0usize;

    for creature in deserialize_json_creatures(json_paths) {
        for action in &creature.actions {
            for issue in find_remaining_tags(&action.description) {
                println!(
                    "[{system}] creature '{}' action '{}': {issue}",
                    creature.name, action.name
                );
                total_issues += 1;
            }
        }
    }

    for item in deserialize_json_items(json_paths) {
        for issue in find_remaining_tags(&item.description) {
            println!("[{system}] item '{}' description: {issue}", item.name);
            total_issues += 1;
        }
    }

    for weapon in deserialize_json_weapons(json_paths) {
        for issue in find_remaining_tags(&weapon.item_core.description) {
            println!(
                "[{system}] weapon '{}' description: {issue}",
                weapon.item_core.name
            );
            total_issues += 1;
        }
    }

    for cond in deserialize_json_conditions(json_paths) {
        for issue in find_remaining_tags(&cond.rule.to_string()) {
            println!("[{system}] condition '{}' rule: {issue}", cond.rule);
            total_issues += 1;
        }
    }

    for armor in deserialize_json_armors(json_paths) {
        for issue in find_remaining_tags(&armor.item_core.description) {
            println!(
                "[{system}] armor '{}' description: {issue}",
                armor.item_core.name
            );
            total_issues += 1;
        }
    }

    for shield in deserialize_json_shields(json_paths) {
        for issue in find_remaining_tags(&shield.item_core.description) {
            println!(
                "[{system}] shield '{}' description: {issue}",
                shield.item_core.name
            );
            total_issues += 1;
        }
    }

    for hazard in deserialize_json_hazards(json_paths) {
        for (label, desc) in [
            ("description", &hazard.description),
            ("disable_description", &hazard.disable_description),
            ("reset_description", &hazard.reset_description),
            ("routine_description", &hazard.routine_description),
        ] {
            for issue in desc.parsing_errors(None) {
                println!("[{system}] hazard '{}' {label}: {issue}", hazard.name);
                total_issues += 1;
            }
        }
        for action in &hazard.actions {
            for issue in find_remaining_tags(&action.description) {
                println!(
                    "[{system}] hazard '{}' action '{}': {issue}",
                    hazard.name, action.name
                );
                total_issues += 1;
            }
        }
    }

    println!("[{system}] dry-run complete: {total_issues} description issue(s) found");
}

#[cfg(not(feature = "dry-run"))]
async fn clear_db(conn: &PgPool) -> anyhow::Result<()> {
    db_handler_one::drop_tables_except(
        conn,
        &[
            "pf_item_table",
            "pf_creature_table",
            "sf_item_table",
            "sf_creature_table",
            "pf_hazard_table",
            "sf_hazard_table",
        ],
    )
    .await
    .unwrap();
    Ok(())
}

#[cfg(not(feature = "dry-run"))]
async fn db_update(
    conn: &PgPool,
    pf2e_json_paths: Vec<String>,
    sf2e_json_paths: Vec<String>,
) -> anyhow::Result<()> {
    let mut tx: Transaction<Postgres> = conn.begin().await.unwrap();

    let pf2e_conditions = deserialize_json_conditions(&pf2e_json_paths);

    db_handler_one::insert_conditions(&mut tx, &GameSystem::Pathfinder, &pf2e_conditions)
        .await
        .unwrap();

    let sf2e_conditions = deserialize_json_conditions(&sf2e_json_paths);

    let sf2e_names: std::collections::HashSet<&str> =
        sf2e_conditions.iter().map(|c| c.name.as_str()).collect();

    let sf2e_conditions: Vec<BybeCondition> = pf2e_conditions
        .iter()
        .filter(|c| !sf2e_names.contains(c.name.as_str()))
        .cloned()
        .chain(sf2e_conditions.clone())
        .collect();

    db_handler_one::insert_conditions(&mut tx, &GameSystem::Starfinder, &sf2e_conditions)
        .await
        .unwrap();

    set_game_system(GameSystem::Pathfinder);
    game_system_tables_update(&mut tx, pf2e_json_paths, &GameSystem::Pathfinder)
        .await
        .unwrap();

    db_handler_one::update_with_aon_data(&mut tx).await.unwrap();
    set_game_system(GameSystem::Starfinder);
    game_system_tables_update(&mut tx, sf2e_json_paths, &GameSystem::Starfinder)
        .await
        .unwrap();

    db_handler_one::insert_scales_values_to_db(&mut tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();
    Ok(())
}

#[cfg(not(feature = "dry-run"))]
async fn game_system_tables_update(
    tx: &mut Transaction<'_, Postgres>,
    json_paths: Vec<String>,
    gs: &GameSystem,
) -> anyhow::Result<()> {
    for el in deserialize_json_hazards(&json_paths) {
        if let Err(e) = db_handler_one::insert_hazard_to_db(tx, gs, &el).await {
            error!(
                "Failed to insert hazard: {:?}, skipping with error {:?}",
                el, e
            );
        };
    }

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
    // creatures are added last to avoid useless duplicates for items, weapons, etc.
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

fn deserialize_json_creatures(json_files: &[String]) -> Vec<BybeCreature> {
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

fn deserialize_json_conditions(json_files: &[String]) -> Vec<BybeCondition> {
    let mut conditions = Vec::new();
    for file in json_files {
        match BybeCondition::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            Ok(item) => conditions.push(item),
            Err(e) => match e {
                BybeConditionParsingError::UnsupportedConditionType => {}
                _ => panic!(
                    "Error parsing item {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    conditions
}

fn deserialize_json_items(json_files: &[String]) -> Vec<BybeItem> {
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
                    "Error parsing item {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    items
}

fn deserialize_json_weapons(json_files: &[String]) -> Vec<BybeWeapon> {
    let mut weapons = Vec::new();
    for file in json_files {
        match SourceWeapon::try_from((
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
            false,
        )) {
            Ok(item) => weapons.push(BybeWeapon::from(item)),
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

fn deserialize_json_armors(json_files: &[String]) -> Vec<BybeArmor> {
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
                    "Error parsing armor {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    armors
}

fn deserialize_json_shields(json_files: &[String]) -> Vec<BybeShield> {
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
                    "Error parsing shield {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    shields
}

fn deserialize_json_hazards(json_files: &[String]) -> Vec<BybeHazard> {
    let mut hazards = Vec::new();
    for file in json_files {
        match SourceHazard::try_from(
            &serde_json::from_str(&read_from_file_to_string(file.as_str()))
                .expect("JSON was not well-formatted"),
        ) {
            Ok(hazard) => hazards.push(BybeHazard::from(hazard)),
            Err(e) => match e {
                SourceHazardParsingError::InvalidHazardType
                | SourceHazardParsingError::HazardTypeFormat => {}
                _ => panic!(
                    "Error parsing hazard {} \n{}",
                    e,
                    backtrace::Backtrace::capture()
                ),
            },
        }
    }
    hazards
}

fn is_dir_empty(path: &str) -> bool {
    fs::read_dir(path)
        .map(|mut e| e.next().is_none())
        .unwrap_or(false)
}

fn fetch_source_data(source_url: &str, source_path: &str) {
    // Clones source if the given path is empty, otherwise warns
    if is_dir_empty(source_path) {
        debug!("Cloning path: {source_path}");
        match git2::build::RepoBuilder::new().clone(source_url, source_path.as_ref()) {
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
