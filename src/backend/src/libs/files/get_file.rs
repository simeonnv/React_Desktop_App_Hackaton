use crate::libs::db::get_pool::get_pool;
use crate::error::Error;
use crate::libs::db::structs::files::Files;

pub async fn get_file(file_id: i32) -> Result<Files, Error> {

    let pool = get_pool();

    
    let file: Option<Files> = sqlx::query_as(r#"
        
        SELECT file_id, file_blob, size, file_type, account_id, created_at
        FROM Files
        WHERE file_id = $1;
        
    "#)
        .bind(file_id)
        .fetch_optional(pool)
        .await?;

    match file {
        Some(e) => Ok(e),
        None => Err(Error::NotFound("file does not exist!".to_string()))
    }
}