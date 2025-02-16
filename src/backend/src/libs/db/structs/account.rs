use chrono::NaiveDateTime;
use sqlx::types::chrono;

#[derive(sqlx::FromRow, Debug)]
pub struct Accounts {
    pub account_id: i32,
    pub pfp_id: Option<i32>,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime
}

