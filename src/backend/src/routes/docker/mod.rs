use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, web, Scope};
use crate::libs::auth::auth_middleware::AuthMiddleware;

pub mod socket_docker_crashes;
pub mod get_docker_containers;
pub mod socket_docker_usage;
pub mod socket_docker_logs;



pub fn docker() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/docker")
        // .wrap(AuthMiddleware)
        
        .service(get_docker_containers::get_docker_containers)
        
        .route("/crashes", web::get().to(socket_docker_crashes::socket_docker_crashes))
        .route("/usage", web::get().to(socket_docker_usage::socket_docker_usage))
        .route("/logs", web::get().to(socket_docker_logs::socket_docker_logs))
        
}
