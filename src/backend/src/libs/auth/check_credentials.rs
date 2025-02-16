use serde::{Deserialize, Serialize};

use crate::libs::crypto::compare::compare;
use crate::error::Error;
use crate::libs::db::get_pool::get_pool;
use crate::libs::db::structs::account::Accounts;



#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}


pub async fn check_credentials(username: &String, password: &String) -> Result<(i32, String), Error> {

    let pool = get_pool();

    let db_res: Option<Accounts> = sqlx::query_as(r#"
        SELECT * FROM Accounts 
            WHERE username = $1
        ;
    "#)
        .bind(username)
        .fetch_optional(pool)
        .await?;

    let account = match db_res {
        Some(value) => value,
        None => return Err(Error::Conflict("Invalid username or password!".to_string()))
    };

    match compare(password, &account.password).await? {
        true => Ok((account.account_id, account.role)),
        false => Err(Error::Conflict("Invalid username or password!".to_string()))
    }
    
}