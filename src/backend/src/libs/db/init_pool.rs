use crate::DB;
use super::init_db::init_db;


pub async fn init_pool() -> Result<(), sqlx::Error> {
    let pool = init_db().await?;
    DB.set(pool).map_err(|_| {
        sqlx::Error::Configuration("Failed to initialize global database pool".into())
    })?;
    Ok(())
}