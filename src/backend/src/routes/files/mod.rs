use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, web, Scope};
use crate::libs::auth::auth_middleware::AuthMiddleware;

// pub mod post_image_id;
pub mod get_files_id;
pub mod post_files_id;



pub fn image() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/image")
        .wrap(AuthMiddleware)

        // .service(post_files_id::post_files_id)
        .service(get_files_id::get_files_id)
        
}
