use bollard::Docker;
use crate::DOCKER;


pub async fn get_docker() -> Docker {
    DOCKER.get().expect("docker is not initialized").clone()
}

