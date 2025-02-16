use serde::{Deserialize, Serialize};

use crate::libs::db::get_pool::get_pool;
use crate::error::Error;


#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}


pub async fn does_account_exist(username: &String) -> Result<(), Error> {

    let pool = get_pool();

    let exists = sqlx::query_scalar(r#"
        SELECT EXISTS (
            SELECT 1 FROM Accounts WHERE username = $1
        );
    "#)
        .bind(username)
        .fetch_one(pool)
        .await?;

    dbg!(&exists);

    match exists {
        false => Ok(()),
        true => Err(Error::Conflict("Account already exists!".to_string()))
    }
    
}