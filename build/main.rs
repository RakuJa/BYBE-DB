use dotenvy::dotenv;
use log::{debug, warn};
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;
use std::{env, fs};

#[tokio::main]
async fn main() {
    dotenv().ok(); // use dotenv env variables
    let db_url = &env::var("DATABASE_URL")
        .expect("DB URL IS NOT SET.. Aborting. Hint: set DATABASE_URL environmental variable");

    let db_path = &env::var("DATABASE_PATH")
        .or_else(|_| env::var("DATABASE_FOLDER_PATH"))
        .expect(
            "DB PATH IS NOT SET.. Aborting. Hint: set DATABASE_FOLDER_PATH environmental variable",
        );
    fs::create_dir_all(db_path).expect("Could not create parent folder to save db.");

    let conn = SqlitePool::connect_with(
        SqliteConnectOptions::from_str(db_url)
            .expect("Could not find a valid db in the given path")
            .create_if_missing(true),
    )
    .await
    .expect("Could not connect to the given db url, something went wrong..");
    match sqlx::migrate!("./migrations").run(&conn).await {
        Ok(_) => debug!("Migrated successfully"),
        Err(e) => warn!("Migrate failed: {}", e),
    }
    conn.close().await;
}
