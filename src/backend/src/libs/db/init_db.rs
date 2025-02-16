use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;
use crate::config;


pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    println!("Connecting to db");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(format!("postgres://{}:{}@{}", config::DB_USERNAME, config::DB_PASSWORD, config::DB_ADDRESS).as_str())
        .await?;

    println!("Connected to postgres://{}:{}@{}", config::DB_USERNAME, config::DB_PASSWORD, config::DB_ADDRESS);

    let _ = sqlx::query(&format!("CREATE DATABASE {}", config::DB_NAME))
        .execute(&pool)
        .await;

    println!("Database '{}' created or already exists!", config::DB_NAME);

    let pool_with_db: Pool<Postgres> = PgPoolOptions::new()
        .connect(
            format!("postgres://{}:{}@{}/{}", 
                config::DB_USERNAME, 
                config::DB_PASSWORD, 
                config::DB_ADDRESS,
                config::DB_NAME
            ).as_str())
        .await?;

    println!("Connected to db {}", config::DB_NAME);

    Ok(pool_with_db)
}


