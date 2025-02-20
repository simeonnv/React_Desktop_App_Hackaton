use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use sqlx::types::chrono::NaiveDateTime;
use crate::error::Error as ErrorHandler;
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::libs::db::get_pool::get_pool;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}
#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct AccountData {
    pub id: i32,
    pub token: String,
    pub role: String,
    pub token_creation_date: NaiveDateTime,
    pub username: String,
    pub account_creation_date: NaiveDateTime,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .map(|auth| auth.to_string());

            if let Some(auth_header) = auth_header {
                if auth_header.starts_with("Bearer ") {
                    let token = String::from(&auth_header["Bearer ".len()..]);
                    let pool = get_pool();

                    let db_res: Option<AccountData> = sqlx::query_as(r#"
                        SELECT
                            Tokens.token,
                            Tokens.role,
                            Tokens.created_at AS token_creation_date,
                            Accounts.username,
                            Accounts.account_id AS id,
                            Accounts.created_at AS account_creation_date
                        FROM
                            Tokens
                        INNER JOIN Accounts ON
                            Tokens.account_id = Accounts.account_id
                        WHERE token = $1;
                    "#)
                        .bind(token)
                        .fetch_optional(pool)
                        .await
                        .map_err(ErrorHandler::from)?; // Use custom error conversion

                    dbg!(&db_res);

                    let account_data = db_res.ok_or_else(|| ErrorHandler::Unauthorized("Token has expired!".to_string()))?;

                    req.extensions_mut().insert(account_data);
                    return service.call(req).await;
                }
            }

            Err(ErrorHandler::Unauthorized("Invalid or missing token".to_string()).into())
        })
    }
}
