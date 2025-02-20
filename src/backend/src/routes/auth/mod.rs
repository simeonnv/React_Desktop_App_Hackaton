use actix_web::{
    web, Scope
};

use crate::libs::auth::auth_middleware::AuthMiddleware;

pub mod post_auth_signup;
pub mod post_auth_login;
pub mod get_auth_validate;

pub fn auth() -> Scope {
    web::scope("/auth")
        .service(post_auth_signup::post_auth_signup)
        .service(post_auth_login::post_auth_login)
        .service(
            web::resource("/validate")
                .wrap(AuthMiddleware) 
                .route(web::get().to(get_auth_validate::get_auth_validate))
        )

}