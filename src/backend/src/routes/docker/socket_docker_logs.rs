use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use bollard::{container::LogsOptions, Docker};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::interval;
use utoipa::ToSchema;

struct WsContainerLogs {
    interval: Duration,
}

impl Actor for WsContainerLogs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // No immediate action needed here; log streaming starts when a container ID is received
    }
}

// Message to send logs to the client
#[derive(Message)]
#[rtype(result = "()")]
struct LogMessage(String);

impl Handler<LogMessage> for WsContainerLogs {
    type Result = ();

    fn handle(&mut self, msg: LogMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket handler for container logs
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsContainerLogs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let container_id = text.trim().to_string();
            let docker = match Docker::connect_with_unix_defaults() {
                Ok(docker) => docker,
                Err(e) => {
                    ctx.text(format!("Error connecting to Docker: {}", e));
                    return;
                }
            };
            let addr = ctx.address();
            let interval_duration = self.interval;

            let options = LogsOptions::<String> {
                follow: true,
                stdout: true,
                stderr: true,
                timestamps: false,
                ..Default::default()
            };

            actix::spawn(async move {
                let mut logs_stream = docker.logs(&container_id, Some(options));
                let mut log_buffer = String::new();
                let mut ticker = interval(interval_duration);

                while let Some(log_result) = logs_stream.next().await {
                    match log_result {
                        Ok(log) => {
                            let message = String::from_utf8_lossy(&log.into_bytes()).to_string();
                            log_buffer.push_str(&message);
                            log_buffer.push('\n'); // Add newline for readability

                            // Wait for the next tick to send buffered logs
                            ticker.tick().await;
                            if !log_buffer.is_empty() {
                                addr.send(LogMessage(log_buffer.clone())).await.ok();
                                log_buffer.clear(); // Clear buffer after sending
                            }
                        }
                        Err(e) => {
                            let error_msg = format!("Error: {}", e);
                            addr.send(LogMessage(error_msg)).await.ok();
                            break;
                        }
                    }
                }

                // Send any remaining logs when the stream ends
                if !log_buffer.is_empty() {
                    addr.send(LogMessage(log_buffer)).await.ok();
                }
            });
        }
    }
}

#[derive(Serialize, ToSchema)]
struct SocketDockerLogsResDocs {
    status: &'static str,
    data: String,
}

#[utoipa::path(
    get,
    path = "/docker/logs",
    responses(
        (status = 101, description = "WebSocket connection established, logs sent as text at specified interval", body = SocketDockerLogsResDocs, example = json!({
            "status": "success",
            "data": "2025-02-22T16:01:49Z [INFO] Container started\n2025-02-22T16:01:50Z [ERROR] Failed to connect to database"
        })),
        (status = 400, description = "Bad request - WebSocket upgrade failed")
    ),
    params(
        ("interval" = Option<u64>, Query, description = "Interval in seconds to send buffered logs, default is 10, clamped between 1 and 60"),
        ("Upgrade" = String, Header, description = "Required: 'websocket'", example = "websocket"),
        ("Connection" = String, Header, description = "Required: 'Upgrade'", example = "Upgrade"),
        ("Sec-WebSocket-Version" = String, Header, description = "Required: '13'", example = "13"),
        ("Sec-WebSocket-Key" = String, Header, description = "Required: '13'", example = "13")
    ),
    tag = "Docker",
    description = "Establishes a WebSocket connection that streams Docker container logs for a specified container ID at the specified interval.\n\nThe client must send a text message with the container ID to start receiving logs. Logs are buffered and sent as text messages."
)]
pub async fn socket_docker_logs(
    req: HttpRequest,
    query: web::Query<WsQuery>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let interval_secs = query.interval.unwrap_or(10).clamp(1, 60);
    let interval_duration = Duration::from_secs(interval_secs);

    ws::start(
        WsContainerLogs {
            interval: interval_duration,
        },
        &req,
        stream,
    )
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WsQuery {
    pub interval: Option<u64>,
}