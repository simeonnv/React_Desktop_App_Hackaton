use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::files::upload_file::upload_file;

#[derive(Serialize, Deserialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<i32>
}


#[derive(Serialize, Deserialize)]
pub struct Req {
    pub file_blob: Vec<u8>,
}


#[utoipa::path(
    post,
    path = "/files",
    request_body = PostFilesIdReqDocs,
    responses(
        (status = 200, description = "Signup successful", body = PostFilesIdResDocs, example = json!({
            "status": "success",
            "data": 16
        })),
        (status = 401, description = "Unauthorized", body = PostFilesIdResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = PostFilesIdResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Files"
)]
#[post("")]
async fn post_files(token_data: HttpRequest, req: web::Json<Req>) -> Result<HttpResponse, Error> {
    // if let Some(account_info) = token_data.extensions().get::<AccountData>() {

        let file_id = upload_file(&req.file_blob, 1).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "success",
            data: Some(file_id),
        }))

    // } else {
    //     return Ok(HttpResponse::Unauthorized().json(Res {
    //         status: "Unauthorized access",
    //         data: None,
    //     }))
    // }
}


#[derive(Serialize, Deserialize, ToSchema)]
struct PostFilesIdResDocs {
    status: &'static str,
    data: Option<i64>
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct PostFilesIdReqDocs {
    pub file_blob: Vec<u8>,
}
