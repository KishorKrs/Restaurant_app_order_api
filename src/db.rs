use std::{fs, env};
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

pub async fn get_pool() ->  Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;
    return Ok(pool);
}

/// Initialize the database by executing the migration file
pub async fn initialize_database() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migration_path = "./migrations/create_orders_table.sql";
    
    // Check if the database exists, create it if it doesn't
    if !Sqlite::database_exists(&database_url).await? {
        println!("Creating database at: {}", database_url);
        Sqlite::create_database(&database_url).await?;
    }

    let pool = SqlitePool::connect(&database_url).await?;

    // Read the migration file and execute
    let migration = fs::read_to_string(migration_path).expect("Failed to read migration file");
    sqlx::query(&migration).execute(&pool).await?;

    return Ok(pool)
}
