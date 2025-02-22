use bollard::Docker;
use crate::{error, DOCKER};


pub async fn init_docker() -> Result<(), error::Error> {
    DOCKER.set(Docker::connect_with_local_defaults()?).map_err(|_| error::Error::Internal("Docker init error".to_string()))?;
    Ok(())
}


