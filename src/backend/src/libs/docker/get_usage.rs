use bollard::{container::{ListContainersOptions, StatsOptions}, Docker};
use serde::Serialize;
use utoipa::ToSchema;
use futures_util::stream::TryStreamExt;
use crate::error;

use super::{get_containers::get_containers, get_docker::get_docker};

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct Stats {
    pub read: String,
    pub names: Vec<String>,
    pub id: String,
    pub memory_stats: MemoryStats,
    pub cpu_stats: CpuStats,
    pub pids_stats: PidsStats,
    pub network: Option<Network>,
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct MemoryStats { pub usage: u64, pub limit: u64 }

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct CpuStats { pub online_cpus: u64, pub total_usage: u64, system_cpu_usage: u64 }

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct PidsStats { pub current: u64, pub limit: u64 }

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct Network {
    pub rx_dropped: u64, pub rx_bytes: u64, pub rx_errors: u64,
    pub tx_packets: u64, pub tx_dropped: u64, pub rx_packets: u64,
    pub tx_errors: u64, pub tx_bytes: u64,
}

pub async fn get_usage() -> Result<Vec<Stats>, error::Error> {
    let containers = get_containers().await?;
    let docker = get_docker().await;
    let options = StatsOptions { stream: false, one_shot: true };

    containers.into_iter()
        .filter(|c| c.state == "running")
        .map(|container| {
        let value = docker.clone();
        async move {
            let stats_raw = value.stats(&container.id, Some(options.clone()))
                .try_next()
                .await
                .map_err(|e| error::Error::Internal(format!("Docker usage error: {}", e)))?
                .ok_or_else(|| error::Error::Internal("No stats available".to_string()))?;

            // dbg!(&stats_raw);

            Ok(Stats {
                read: stats_raw.read,
                names: container.names,
                id: container.id,
                memory_stats: MemoryStats {
                    usage: stats_raw.memory_stats.usage.unwrap_or_default(),
                    limit: stats_raw.memory_stats.limit.unwrap_or_default(),
                },
                cpu_stats: CpuStats {
                    online_cpus: stats_raw.cpu_stats.online_cpus.unwrap_or_default(),
                    total_usage: stats_raw.cpu_stats.cpu_usage.total_usage,
                    system_cpu_usage: stats_raw.cpu_stats.system_cpu_usage.unwrap_or_default()
                },
                pids_stats: PidsStats {
                    current: stats_raw.pids_stats.current.unwrap_or_default(),
                    limit: stats_raw.pids_stats.limit.unwrap_or_default(),
                },
                network: stats_raw.networks
                    .and_then(|n| n.get("eth0").cloned())
                    .map(|n| Network {
                        rx_dropped: n.rx_dropped, rx_bytes: n.rx_bytes, rx_errors: n.rx_errors,
                        tx_packets: n.tx_packets, tx_dropped: n.tx_dropped, rx_packets: n.rx_packets,
                        tx_errors: n.tx_errors, tx_bytes: n.tx_bytes,
                    }),
            })
        }
        })
        .collect::<futures::future::JoinAll<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
}