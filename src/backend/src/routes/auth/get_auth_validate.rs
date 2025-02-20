use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<User>
}
#[derive(Serialize, Debug)]
struct User {
    username: String,
    id: i32
}


#[utoipa::path(
    get,
    path = "/auth/validate",
    responses(
        (status = 200, description = "auth successful", body = GetAuthValidateResDocs, example = json!({
            "status": "success",
            "data": {
                "username": "NAME",
                "id": 15
            }
        })),
        (status = 401, description = "Unauthorized", body = GetAuthValidateResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Auth"
)]

pub async fn get_auth_validate(
    token_data: HttpRequest,
) -> Result<HttpResponse, Error> {
    match token_data.extensions().get::<AccountData>() {
        Some(e) => 
            Ok(HttpResponse::Ok().json(Res {
                status: "success",
                data: Some( User {
                    username: e.username.clone(),
                    id: e.id
                }),
            })),
        None =>
            return Ok(HttpResponse::Unauthorized().json(Res {
                status: "Unauthorized access",
                data: None,
            }))
    }
}

#[derive(Serialize, ToSchema)]
struct GetAuthValidateResDocs {
    status: &'static str,
    data: Option<GetAuthValidateUserDocs>
}

#[derive(Serialize, ToSchema)]
struct GetAuthValidateUserDocs {
    username: String,
    id: i32
}