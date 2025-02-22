use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix::AsyncContext;
use bollard::{container::LogsOptions, Docker};
use futures_util::StreamExt;
use std::collections::HashMap;
use tokio::{io::{AsyncBufReadExt, BufReader}, sync::futures};

struct WsContainerLogs;

impl actix::Actor for WsContainerLogs {
    type Context = ws::WebsocketContext<Self>;
}

// Add message handler
#[derive(actix::Message)]
#[rtype(result = "()")]
struct LogMessage(String);

impl actix::Handler<LogMessage> for WsContainerLogs {
    type Result = ();

    fn handle(&mut self, msg: LogMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket handler for container logs
impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsContainerLogs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        if let Ok(ws::Message::Text(text)) = msg {
            let container_id = text.trim().to_string();
            let docker = Docker::connect_with_unix_defaults().unwrap();
            
            // Get the context address
            let addr = ctx.address();
            
            let options = LogsOptions::<String> {
                follow: true,
                stdout: true,
                stderr: true,
                timestamps: false,
                ..Default::default()
            };

            actix::spawn(async move {
                let mut logs_stream = docker.logs(&container_id, Some(options));
                
                while let Some(log_result) = logs_stream.next().await {
                    match log_result {
                        Ok(log) => {
                            let message = String::from_utf8_lossy(&log.into_bytes()).to_string();
                            // Send custom message to actor
                            addr.send(LogMessage(message)).await.ok();
                        }
                        Err(e) => {
                            let error_msg = format!("Error: {}", e);
                            addr.send(LogMessage(error_msg)).await.ok();
                            break;
                        }
                    }
                }
            });
        }
    }
}

pub async fn socket_docker_logs_id(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsContainerLogs {}, &req, stream)
}