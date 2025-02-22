use bollard::{container::ListContainersOptions, secret::ContainerSummary, Docker};
use crate::error;

use super::get_docker::get_docker;


pub async fn get_raw_containers() -> Result<Vec<ContainerSummary>, error::Error> {
    let docker = get_docker().await;

    let containers = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true, // Include stopped containers
            ..Default::default()
        }))
        .await
        .map_err(|e| error::Error::Internal(format!("Docker cant list containers: {}", e)))?;


    Ok(containers)
}


