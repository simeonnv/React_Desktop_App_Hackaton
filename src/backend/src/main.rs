use std::sync::Arc;

use actix_cors::Cors;
use chrono::{DateTime, Local};
// use libs::auth::create_account::create_account;
use libs::{auth::create_account::create_account, db};
use routes::auth::auth;
use tokio::sync::{Mutex, OnceCell};

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api_docs;
pub mod config;
pub mod error;
pub mod libs;
pub mod routes;

static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();
type SharedSessions = Arc<Mutex<Vec<Session>>>;
pub struct Session {
    started: DateTime<Local>,
    uuid: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    db::init_pool::init_pool().await.expect("Failed to initialize database");
    db::init_tables::init_tables().await.expect("Failed to initialize tables");

    let _ = create_account(&"admin".to_string(), &"admin".to_string(), "admin").await;

    

    HttpServer::new(|| {
                
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            
            .service(auth())

            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()))
    })
    .bind((config::LISTENING_ON, config::PORT))?
    .run()
    .await
}