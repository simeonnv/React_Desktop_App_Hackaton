use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::interval;
use utoipa::ToSchema;

use crate::libs::docker::get_crashes::{get_crashes, CrashedContainer};

// WebSocket actor
struct MyWebSocket {
    hb: Instant,
    interval: Duration,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb = Instant::now();
        
        let addr = ctx.address();
        let interval_duration = self.interval; // Copy the duration for the async block
        actix_rt::spawn(async move {
            let mut interval = interval(interval_duration);
            
            loop {
                interval.tick().await;
                
                match get_crashes().await {
                    Ok(containers) => {
                        if let Ok(json) = serde_json::to_string(&containers) {
                            addr.do_send(WsMessage(json));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting crashes: {:?}", e);
                    }
                }
            }
        });
    }
}

// Message type for sending data to websocket
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);

impl Handler<WsMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

#[derive(Serialize, ToSchema)]
struct SocketDockerCrashesResDocs {
    status: &'static str,
    data: Vec<CrashedContainer>,
}

#[utoipa::path(
    get,
    path = "/docker/crashes",
    responses(
        (status = 101, description = "WebSocket connection established", body = SocketDockerCrashesResDocs, example = json!({
            "status": "success",
            "data": [
                {
                  "container_id": "4e678563469271a6c7ae113be6b3a3bc3f627c6f7646e0df2d44250e5b55fd10",
                  "container_names": [
                    "/practical_cohen"
                  ],
                  "running": false,
                  "exit_code": 101,
                  "status": "exited",
                  "error": "",
                  "started_at": "2025-02-10T18:18:24.06822891Z",
                  "finished_at": "2025-02-10T18:18:54.389672353Z",
                  "died_to_oom": false
                }
            ]
        })),
        (status = 400, description = "Bad request - WebSocket upgrade failed")
    ),
    params(
        ("interval" = Option<u64>, Query, description = "Interval in seconds to send crashed container data, default is 10, clamped between 1 and 60"),
        ("Upgrade" = String, Header, description = "Required: 'websocket'", example = "websocket"),
        ("Connection" = String, Header, description = "Required: 'Upgrade'", example = "Upgrade"),
        ("Sec-WebSocket-Version" = String, Header, description = "Required: '13'", example = "13"),
        ("Sec-WebSocket-Key" = String, Header, description = "Required: '13'", example = "13")
    ),
    tag = "Docker",
    description = "Establishes a WebSocket connection that streams crashed container data at the specified interval.\n\nMessages are sent as JSON arrays of CrashedContainer objects."
)]
pub async fn socket_docker_crashes(
    req: HttpRequest,
    query: web::Query<WsQuery>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let interval_secs = query.interval.unwrap_or(10).clamp(1, 60);
    let interval_duration = Duration::from_secs(interval_secs);

    ws::start(
        MyWebSocket {
            hb: Instant::now(),
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