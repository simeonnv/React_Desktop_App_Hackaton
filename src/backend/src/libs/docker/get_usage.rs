use bollard::{container::{ListContainersOptions, StatsOptions}, secret::ContainerSummary, Docker};
use crate::error;
use futures_util::stream::TryStreamExt; // Updated import

use super::{get_containers::get_containers, get_docker::get_docker};

pub async fn get_usage() -> Result<Vec<bollard::container::Stats>, error::Error> {
    let containers = get_containers().await?;
    let docker = get_docker().await;
    
    let options = StatsOptions {
        stream: false,
        one_shot: true,
    };

    let mut results = Vec::new();
    
    for container in containers {
        println!("Container ID: {}", container.id);

        let stats = docker
            .stats(&container.id, Some(options.clone()))
            .try_next()  // Get the first (and only) item from the stream
            .await
            .map_err(|e| error::Error::Internal(format!("Docker usage error: {}", e)))?;
        match stats {
            Some(e) => results.push(e),
            None => continue
        };
    }
    
    Ok(results)
}