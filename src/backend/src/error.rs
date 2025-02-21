use std::{fmt, str::Utf8Error, string::FromUtf8Error};

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use argon2::password_hash;

#[derive(Serialize, Deserialize)]
pub struct ErrorRes {
    status: String,
    data: &'static str
}

#[derive(Debug)]
pub enum Error {
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    UniqueNameViolation(String),
    NotFound(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Error::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Error::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::UniqueNameViolation(msg) => write!(f, "Unique constraint violation: {}", msg),
            Error::NotFound(msg) => write!(f, "Not Found: {}", msg),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Conflict(msg) => HttpResponse::Conflict().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::Internal(msg) => HttpResponse::InternalServerError().json(ErrorRes {
                status: format!("server skillissue: {}", msg),
                data: "",
            }),
            Error::UniqueNameViolation(msg) => HttpResponse::Conflict().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::NotFound(msg) => HttpResponse::NotFound().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
        }
    }
}


impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.code().as_deref() == Some("23000") { // SQLSTATE for integrity constraint violation
                if let Some(message) = db_err.message().split("Duplicate entry").nth(1) {
                    return Error::UniqueNameViolation(format!("Name has already been used: {}", message.trim()));
                }
            }
        }
        Error::Internal(format!("Database error: {}", err))
    }
}

impl From<password_hash::Error> for Error {
    fn from(err: password_hash::Error) -> Self {
        Error::Internal(format!("Crypto hash error: {}", err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(format!("IO error: {}", err))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Internal(format!("UTF-8 conversion error: {}", err))
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Internal(format!("UTF-8 conversion error: {}", err))
    }
}