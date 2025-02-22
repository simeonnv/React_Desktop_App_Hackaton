use std::string;

use bollard::{container::InspectContainerOptions, secret::ContainerStateStatusEnum, Docker};
use crate::{error, DOCKER};

use super::{get_containers::get_containers, get_docker::get_docker};

pub struct CrashedContainer {
    container_id: String,
    container_name: String,
    running: bool,
    exit_code: i64,
    status: ContainerStateStatusEnum,
    error: String,
    started_at: String,
    finished_at: String,
    died_to_OOM: bool
}


pub async fn get_errors() -> Result<Vec<CrashedContainer>, error::Error> {
    
    let containers = get_containers().await?;
    let docker = get_docker().await;
    let mut crashed_containers: Vec<CrashedContainer> = Vec::<CrashedContainer>::new();

    for container in containers {
        let container_id = container.id.expect("Container ID missing");
        let container_name = container.names.unwrap_or(vec!["Unnamed".to_string()]).join(", ");

        // Inspect the container for detailed state
        match docker
            .inspect_container(&container_id, Some(InspectContainerOptions { size: true }))
            .await
        {
            Ok(container_info) => {
                if let Some(state) = container_info.state {
                    let running = state.running.unwrap_or(false);
                    let exit_code = state.exit_code.unwrap_or(0);
                    let status = state.status.unwrap_or(ContainerStateStatusEnum::EMPTY);
                    let error = state.error.unwrap_or("None".to_string());
                    let started_at = state.started_at.unwrap_or_default();
                    let finished_at = state.finished_at.unwrap_or_default();
                    let died_to_OOM = state.oom_killed.unwrap_or(false);
                    
                    // Check if the container crashed or stopped abnormally
                    if !running && (exit_code != 0 && status == ContainerStateStatusEnum::EXITED || !error.is_empty()) {
                        
                        crashed_containers.push(CrashedContainer {
                            container_id: container_id,
                            container_name: container_name,
                            running: running,
                            exit_code: exit_code,
                            status: status,
                            error: error,
                            started_at: started_at,
                            finished_at: finished_at,
                            died_to_OOM: died_to_OOM
                        });

                    }
                }
            }
            Err(e) => eprintln!("Failed to inspect container {}: {}", container_id, e),
        }
    }

    Ok(crashed_containers)

}


