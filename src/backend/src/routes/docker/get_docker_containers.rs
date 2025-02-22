use actix_web::{get, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::docker;
use crate::libs::docker::get_containers::Container;

#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Vec<Container>,
}

#[utoipa::path(
    get,
    path = "/docker/containers",
    responses(
        (status = 200, description = "auth successful", body = GetDockerContainerResDocs, example = json!({
            "status": "success",
            "data": [
            {
              "id": "f4d61427cc6d275cde3ce044a28f3fb646438f2dc92bad26d71807ff87454ef0",
              "names": [
                "/react_desktop_app_hackaton-hackaton_backend-1"
              ],
              "image": "react_desktop_app_hackaton-hackaton_backend",
              "image_id": "sha256:6331329141b77ea7a9ad182bf3e9020ada521fa87a6678143f542b721cec2619",
              "command": "./hackaton_backend",
              "created": 1740219892,
              "ports": [
                {
                  "IP": "0.0.0.0",
                  "PrivatePort": 6004,
                  "PublicPort": 6004,
                  "Type": "tcp"
                },
                {
                  "IP": "::",
                  "PrivatePort": 6004,
                  "PublicPort": 6004,
                  "Type": "tcp"
                }
              ],
              "size_rw": 0,
              "size_root_fs": 0,
              "state": "running",
              "status": "Up 1 second"
            }]
        }))
    ),
    tag = "Docker"
)]
#[get("/containers")]
pub async fn get_docker_containers() -> Result<HttpResponse, Error> {
    let containers = docker::get_containers::get_containers().await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: containers,
    }));
}

#[derive(Default, Debug, Clone, PartialEq, ToSchema)]
struct GetDockerContainerResDocs {
    status: &'static str,
    data: Vec<GetDockerContainerContainerResDocs>,
}

#[derive(Default, Debug, Clone, PartialEq, ToSchema)]
struct GetDockerContainerContainerResDocs {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub image_id: String,
    pub command: String,
    pub created: i64,
    pub ports: Vec<GetDockerContainerPortResDocs>,
    pub size_rw: i64,
    pub size_root_fs: i64,
    pub state: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, ToSchema)]
struct GetDockerContainerPortResDocs {
    pub ip: String,
    pub private_port: i64,
    pub public_port: i64,
    pub type_field: String,
}
