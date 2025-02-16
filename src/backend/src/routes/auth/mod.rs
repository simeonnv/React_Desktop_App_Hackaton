use actix_web::{
    web, Scope
};

pub mod signup;
pub mod login;

pub fn auth() -> Scope {
    web::scope("/auth")
        .service(signup::signup)
        .service(login::login)

}