use std::collections::HashMap;

use bollard::{container::ListContainersOptions, secret::{ContainerSummary, ContainerSummaryHostConfig, ContainerSummaryNetworkSettings, MountPoint, Port}, Docker};
use serde::Serialize;
use crate::error;

use super::{get_docker::get_docker, get_raw_containers::get_raw_containers};

#[derive(Serialize, Debug)]
pub struct Container {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub image_id: String,
    pub command: String,
    pub created: i64,
    pub ports: Vec<Port>,
    pub size_rw: i64,
    pub size_root_fs: i64,
    pub state: String,
    pub status: String
}

pub async fn get_containers() -> Result<Vec<Container>, error::Error> {
    let raw_containers = get_raw_containers().await?;
    let mut containers: Vec<Container> = Vec::<Container>::new();

    for raw_container in raw_containers {
        if raw_container.id.is_none() {
            continue;
        }

        let container = Container {
            id: raw_container.id.unwrap_or_default(),
            names: raw_container.names.unwrap_or_default(),
            image: raw_container.image.unwrap_or_default(),
            image_id: raw_container.image_id.unwrap_or_default(),
            command: raw_container.command.unwrap_or_default(),
            created: raw_container.created.unwrap_or_default(),
            ports: raw_container.ports.unwrap_or_default(),
            size_rw: raw_container.size_rw.unwrap_or_default(),
            size_root_fs: raw_container.size_root_fs.unwrap_or_default(),
            state: raw_container.state.unwrap_or_default(),
            status: raw_container.status.unwrap_or_default()
        };

        containers.push(container);

    }


    Ok(containers)
}


