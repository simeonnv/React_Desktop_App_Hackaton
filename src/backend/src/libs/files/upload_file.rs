use crate::libs::db::get_pool::get_pool;
use crate::error::Error;

pub async fn upload_file(file_blob: &Vec<u8>, account_id: i32) -> Result<i32, Error> {

    let pool = get_pool();

    let file_type = infer::get(file_blob)
        .map(|t| t.mime_type()) // Get MIME type (e.g., "image/png")
        .unwrap_or("unknown");

    let file_size = file_blob.len() as i32;
    
    let file_id: i32 = sqlx::query_scalar(r#"
        
        INSERT INTO Files
        (file_blob, size, file_type, account_id)
        VALUES($1, $2, $3, $4)
        RETURNING file_id;
        
    "#)
        .bind(file_blob)
        .bind(file_size)
        .bind(file_type)
        .bind(account_id)
        .fetch_one(pool)
        .await?;
    
    Ok(file_id)
}