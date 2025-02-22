use std::string;

use bollard::{container::InspectContainerOptions, secret::ContainerStateStatusEnum, Docker};
use utoipa::ToSchema;
use crate::{error, DOCKER};

use super::{get_containers::get_containers, get_docker::get_docker};

#[derive(sqlx::FromRow, Debug, serde::Serialize, ToSchema)]
pub struct CrashedContainer {
    pub container_id: String,
    pub container_names: Vec<String>,
    pub running: bool,
    pub exit_code: i64,
    pub status: i8,
    pub error: String,
    pub started_at: String,
    pub finished_at: String,
    pub died_to_oom: bool
}


pub async fn get_crashes() -> Result<Vec<CrashedContainer>, error::Error> {
    
    let containers = get_containers().await?;
    let docker = get_docker().await;
    let mut crashed_containers: Vec<CrashedContainer> = Vec::<CrashedContainer>::new();

    for container in containers {
        match docker
            .inspect_container(&container.id, Some(InspectContainerOptions { size: true }))
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
                    let died_to_oom = state.oom_killed.unwrap_or(false);
                    
                    // Check if the container crashed or stopped abnormally
                    if !running && (exit_code != 0 && status == ContainerStateStatusEnum::EXITED || !error.is_empty()) {
                        
                        crashed_containers.push(CrashedContainer {
                            container_id: container.id,
                            container_names: container.names,
                            running: running,
                            exit_code: exit_code,
                            status: 6,
                            error: error,
                            started_at: started_at,
                            finished_at: finished_at,
                            died_to_oom: died_to_oom
                        });

                    }
                }
            }
            Err(e) => eprintln!("Failed to inspect container {}: {}", container.id, e),
        }
    }

    Ok(crashed_containers)

}


