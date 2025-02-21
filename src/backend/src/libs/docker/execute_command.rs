use std::str;

use tokio::process::Command;
use crate::error::Error;


pub async fn execute_command(args: &'static str) -> Result<String, Error> {
    let output = Command::new("docker")
        .arg(args)
        .output()
        .await?;

    let stdout = match output.status.success() {
        true => String::from_utf8(output.stdout)?,
        false => return Err(Error::Internal(String::from_utf8(output.stderr)?))
    };

    Ok(stdout)
}