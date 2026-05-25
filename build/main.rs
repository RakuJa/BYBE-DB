use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let Ok(db_url) = env::var("DATABASE_URL") else {
        return;
    };

    let Ok(conn) = PgPool::connect(&db_url).await else {
        return;
    };

    if let Ok((count,)) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(&conn)
        .await
    {
        println!("cargo:warning=Migrations in table: {count}");
    }

    conn.close().await;
}
