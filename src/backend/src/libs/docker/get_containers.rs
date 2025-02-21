use std::str;

use crate::error::Error;

use super::execute_command::execute_command;


pub async fn get_containers() -> Result<String, Error> {
    // Use `--format` to get JSON output
    let raw_output = execute_command("ps --format '{{json .}}'").await?;

    // Parse the JSON output
    let containers: Vec<serde_json::Value> = serde_json::from_str(&raw_output)?;

    for container in containers {
        let container_id = container["ID"].as_str().unwrap_or("N/A");
        let image = container["Image"].as_str().unwrap_or("N/A");
        let status = container["Status"].as_str().unwrap_or("N/A");

        println!("Container ID: {}, Image: {}, Status: {}", container_id, image, status);
    }

    Ok("Containers processed successfully".to_string())
}