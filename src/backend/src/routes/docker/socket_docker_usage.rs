use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use bollard::secret::ContainerStateStatusEnum;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::interval;
use utoipa::ToSchema;

use crate::libs::docker::{
    get_crashes::{get_crashes, CrashedContainer},
    get_usage::{get_usage, Stats},
};

struct MyWebSocket {
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb = Instant::now();

        let addr = ctx.address();
        actix_rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;

                match get_usage().await {
                    Ok(containers) => {
                        if let Ok(json) = serde_json::to_string(&containers) {
                            addr.do_send(WsMessage(json));
                        }
                    }
                    Err(e) => {
                        // Handle error - maybe log it
                        eprintln!("Error getting usage: {:?}", e);
                    }
                }
            }
        });
    }
}

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
struct SocketDockerUsageResDocs {
    status: &'static str,
    data: Vec<Stats>,
}

#[utoipa::path(
    get,
    path = "/docker/usage",
    responses(
        (status = 101, description = "WebSocket connection established", body = SocketDockerUsageResDocs, example = json!({
            "status": "success",
            "data": [{"read":"2025-02-22T16:01:49.99635365Z","names":["/react_desktop_app_hackaton-hackaton_frontend-1"],"id":"6bede12b8d3146ee50cd30572ad9f85ec984881bb72dcb30546671602bde1660","memory_stats":{"usage":246980608,"limit":3328640},"cpu_stats":{"online_cpus":16,"total_usage":1418034000},"pids_stats":{"current":85,"limit":38202},"network":{"rx_dropped":0,"rx_bytes":11274,"rx_errors":0,"tx_packets":18,"tx_dropped":0,"rx_packets":36,"tx_errors":0,"tx_bytes":4407}},{"read":"2025-02-22T16:01:49.996334504Z","names":["/react_desktop_app_hackaton-hackaton_backend-1"],"id":"9cc7596fde34ce2e6a517568972c4e340938ed51d472094e0396cf873aca178b","memory_stats":{"usage":16330752,"limit":33640},"cpu_stats":{"online_cpus":16,"total_usage":58720000},"pids_stats":{"current":19,"limit":38202},"network":{"rx_dropped":0,"rx_bytes":12633,"rx_errors":0,"tx_packets":98,"tx_dropped":0,"rx_packets":97,"tx_errors":0,"tx_bytes":11559}},{"read":"2025-02-22T16:01:49.996334213Z","names":["/react_desktop_app_hackaton-hackaton_database-1"],"id":"ce479d96c38b1d671c629ae20946d9b51258a42f75282df7728aa43550f1a007","memory_stats":{"usage":30339072,"limit":33428640},"cpu_stats":{"online_cpus":16,"total_usage":153034000},"pids_stats":{"current":8,"limit":38202},"network":{"rx_dropped":0,"rx_bytes":13889,"rx_errors":0,"tx_packets":74,"tx_dropped":0,"rx_packets":115,"tx_errors":0,"tx_bytes":9262}}]
        })),
        (status = 400, description = "Bad request - WebSocket upgrade failed")
    ),
    params(
        ("Upgrade" = String, Header, 
            description = "Required: 'websocket'",
            example = "websocket"),
        ("Connection" = String, Header,
            description = "Required: 'Upgrade'",
            example = "Upgrade"),
        ("Sec-WebSocket-Version" = String, Header,
            description = "Required: '13'",
            example = "13"),
        ("Sec-WebSocket-Key" = String, Header,
            description = "Required: '13'",
            example = "13")
    ),
    tag = "Docker",
    description = "Establishes a WebSocket connection that streams crashed container data every 10 seconds.\n\nMessages are sent as JSON arrays of CrashedContainer objects."
)]
pub async fn socket_docker_usage(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket { hb: Instant::now() }, &req, stream)
}
