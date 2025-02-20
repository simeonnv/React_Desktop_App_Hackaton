
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{error::Error, libs::{auth::{create_account::create_account, create_token::create_token, does_account_exist::does_account_exist}, util::insure_len::insure_len}};

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
struct Res {
    status: &'static str,
    data: String
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = PostAuthSignupDocReq,
    responses(
        (status = 200, description = "Signup successful", body = PostAuthSignupDocsRes, example = json!({
            "status": "success",
            "token": "abc123xyz456"
        })),
        (status = 401, description = "Unauthorized", body = PostAuthSignupDocsRes, example = json!({
            "status": "incorrect credential",
            "token": ""
        })),
        (status = 409, description = "Conflict", body = PostAuthSignupDocsRes, example = json!({
            "status": "account already exists",
            "token": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/signup")]
pub async fn post_auth_signup(req: web::Json<Req>) -> Result<HttpResponse, Error> {

    insure_len(&req.username, 5, 15)?;
    insure_len(&req.password, 5, 30)?;
    
    does_account_exist(&req.username).await?;

    let account_id = create_account(&req.username, &req.password, "user").await?;

    let token: String = create_token(&account_id, "user".to_string()).await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: token,
    }));
   
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Signup Request")]
pub struct PostAuthSignupDocReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(title = "Signup Response")]
struct PostAuthSignupDocsRes {
    status: String,
    token: String
}