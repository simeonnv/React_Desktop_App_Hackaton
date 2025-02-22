use bollard::{container::{ListContainersOptions, StatsOptions}, secret::ContainerSummary, Docker};
use serde::Serialize;
use utoipa::ToSchema;
use crate::error;
use futures_util::stream::TryStreamExt; // Updated import

use super::{get_containers::get_containers, get_docker::get_docker};

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct Stats {
    pub read: String,
    pub names: Vec<String>,
    pub id: String,
    pub memory_stats: MemoryStats,
    pub cpu_stats: CpuStats,
    pub pids_stats: PidsStats
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct MemoryStats {
    pub usage: u64,
    pub limit: u64
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct CpuStats {
    pub online_cpus: u64,
    pub total_usage: u64
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct PidsStats {
    pub current: u64,
    pub limit: u64
}

pub async fn get_usage() -> Result<Vec<Stats>, error::Error> {
    let containers = get_containers().await?;
    let docker = get_docker().await;
    
    let options = StatsOptions {
        stream: false,
        one_shot: true,
    };

    let mut results = Vec::new();
    
    for container in containers {
        println!("Container ID: {}, {}, {}", container.id, container.state, container.status);
        
        if container.state == "running" {
            let stats_raw = match docker
                .stats(&container.id, Some(options.clone()))
                .try_next()
                .await
                .map_err(|e| error::Error::Internal(format!("Docker usage error: {}", e)))? 
            {
                Some(stats) => stats,
                None => continue, // This ensures we skip the loop iteration when stats are unavailable
            };

                
            
            
            let memory_stats = MemoryStats {
                usage: stats_raw.memory_stats.usage.unwrap_or_default(),
                limit: stats_raw.memory_stats.limit.unwrap_or_default()
            };

            let cpu_stats = CpuStats {
                online_cpus: stats_raw.cpu_stats.online_cpus.unwrap_or_default(),
                total_usage: stats_raw.cpu_stats.cpu_usage.total_usage
            };

            let pids_stats = PidsStats {
                current: stats_raw.pids_stats.current.unwrap_or_default(),
                limit: stats_raw.pids_stats.limit.unwrap_or_default()
            };

            let stats = Stats {
                read: stats_raw.read,
                names: container.names,
                id: container.id,
                memory_stats: memory_stats,
                cpu_stats: cpu_stats,
                pids_stats: pids_stats
            };

            dbg!(&stats);

            results.push(stats);
        }
    }
    
    Ok(results)
}