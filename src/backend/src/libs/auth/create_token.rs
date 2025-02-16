
use serde::{Deserialize, Serialize};

use crate::{libs::crypto::rand_string::rand_string, libs::db::get_pool::get_pool, error::Error};

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}

pub async fn create_token(account_id: &i32, role: String) -> Result<String, Error> {

    let pool = get_pool();

    let token = rand_string(255);

    sqlx::query(r#"
        
        INSERT INTO Tokens 
            (role, token, account_id)
            VALUES ($1, $2, $3)
        RETURNING account_id;

    "#)
        .bind(role)
        .bind(&token)
        .bind(account_id)
        .fetch_one(pool)
        .await?;

    Ok(token)
}