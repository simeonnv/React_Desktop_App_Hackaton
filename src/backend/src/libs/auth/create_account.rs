use serde::{Deserialize, Serialize};

use crate::{libs::crypto::hash::hash, libs::db::get_pool::get_pool};
use crate::error::Error;

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}

pub async fn create_account(username: &String, password: &String, role: &'static str) -> Result<i32, Error> {

    let pool = get_pool();

    let hashed_password = hash(password).await?;

    let account_id: i32 = sqlx::query_scalar(r#"

        INSERT INTO Accounts 
            (role, username, password)
            VALUES ($1, $2, $3)
        RETURNING account_id;

    "#)
        .bind(role)
        .bind(username)
        .bind(hashed_password)
        .fetch_one(pool)
        .await?;
    
    Ok(account_id)
}