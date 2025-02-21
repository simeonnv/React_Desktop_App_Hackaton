use actix_web::{get, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::does_account_exist::does_account_exist;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: bool
}


#[utoipa::path(
    get,
    path = "/auth/exists",
    responses(
        (status = 200, description = "auth successful", body = GetAuthExistsResDocs, example = json!({
            "status": "success",
            "data": "true or false"
        }))
    ),
    tag = "Auth"
)]
#[get("/exists")]
pub async fn get_auth_exists() -> Result<HttpResponse, Error> {
    let exists: bool = does_account_exist().await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: exists,
    }));
}

#[derive(Serialize, ToSchema)]
struct GetAuthExistsResDocs {
    status: &'static str,
    data: bool
}