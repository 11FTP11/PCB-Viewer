use validator::ValidationErrors;
use serde_json::json;
use sqlx::Error as SqlxError;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use std::time::SystemTimeError;
use std::fmt;
use axum::http::header::{InvalidHeaderValue, ToStrError};
use serde_json::Error as SerdeJsonError;
use std::io::Error as IOError;
use axum::extract::multipart::MultipartError;
use crate::server::ParserError;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response}
};

pub enum ApiError {
    ValidationError(ValidationErrors),
    ServiceError(ServiceError),
    HttpError(InvalidHeaderValue),
    BadRequest(&'static str),
    FileReadError(MultipartError),
    NotFound,
    Unauthorized,
    Forbidden
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ValidationError(errs) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": "validation_error",
                    "errors": errs.errors() 
                }))
            ).into_response(),

            ApiError::ServiceError(err) => {
                println!("SERVICE ERROR @ {}", err);
                match err {
                    ServiceError::InvalidCredentials => (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "code": err.error_code(),
                            "message": err.to_string()
                        }))
                    ).into_response(),

                    ServiceError::NotFound => (
                        StatusCode::NOT_FOUND,
                        Json(json!({
                            "code": err.error_code(),
                            "message": err.to_string()
                        }))
                    ).into_response(),

                    ServiceError::EmailIsUsed => (
                        StatusCode::CONFLICT,
                        Json(json!({
                            "code": err.error_code(),
                            "message": err.to_string()
                        }))
                    ).into_response(),

                    ServiceError::ParsingJsonError(_) => (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        Json(json!({
                            "code": err.error_code(),
                            "message": "Internal server error"
                        }))
                    ).into_response(),

                    ServiceError::PcadParserError(_) => (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        Json(json!({
                            "code": err.error_code(),
                            "message": "Invalid pcad .pcb file"
                        }))
                    ).into_response(),

                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "code": err.error_code(), 
                            "message": "Internal server error"
                        }))
                    ).into_response()
                }
            },

            ApiError::HttpError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": "http_error", 
                    "message": "Internal server error"
                }))
            ).into_response(),

            ApiError::FileReadError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": "file_read_error",
                    "message": "Error while reading file" 
                }))
            ).into_response(),

            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "code": "not_found",
                    "message": "Resource not found"
                }))
            ).into_response(),

            ApiError::BadRequest(msg) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "bad_request",
                    "message": msg.to_string()
                }))
            ).into_response(),

            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "unauthorized",
                    "message": "Authentication required"
                }))
            ).into_response(),

            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "code": "forbidden",
                    "message": "You don't have permissions to access to this resource"
                }))
            ).into_response(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ValidationError(e) => write!(f, "{}", e),
            ApiError::ServiceError(e) => write!(f, "Service Error: {}", e),
            ApiError::NotFound => write!(f, "not_found"),
            ApiError::Unauthorized => write!(f, "unathorized"), 
            ApiError::HttpError(e) => write!(f, "HTTP error: {}", e),
            ApiError::Forbidden => write!(f, "forbidden"),
            ApiError::BadRequest(e) => write!(f, "{}", e),
            ApiError::FileReadError(e) => write!(f, "{}", e)
        }
    }
}


impl From<ToStrError> for ApiError {
    fn from(_: ToStrError) -> Self {
        ApiError::BadRequest("Invalid header")
    }
}

impl From<MultipartError> for ApiError {
    fn from(e: MultipartError) -> Self {
        ApiError::FileReadError(e)
    }
}

impl From<InvalidHeaderValue> for ApiError {
    fn from(e: InvalidHeaderValue) -> Self {
        ApiError::HttpError(e)
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(e: ValidationErrors) -> Self {
        ApiError::ValidationError(e)
    }
}

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        ApiError::ServiceError(e)
    }
}


pub enum ServiceError {
    DbConError(SqlxError),
    CrypterError(BcryptError),
    TokenError(JwtError),
    TimeError(SystemTimeError),
    ParsingJsonError(SerdeJsonError),
    PcadParserError(ParserError),
    IOError(IOError),
    InvalidCredentials,
    EmailIsUsed,
    NotFound
}

impl ServiceError {
    pub fn error_code(&self) -> &'static str {
        match self {
            ServiceError::DbConError(_) => "database_error",
            ServiceError::CrypterError(_) => "encryption_error",
            ServiceError::TokenError(_) => "jwt_error",
            ServiceError::TimeError(_) => "system_time_error",
            ServiceError::InvalidCredentials => "invalid_credentials",
            ServiceError::NotFound => "info_not_found",
            ServiceError::ParsingJsonError(_) => "json_parsing_error",
            ServiceError::PcadParserError(_) => "pcad_parser_error",
            ServiceError::EmailIsUsed => "email_used",
            ServiceError::IOError(_)=> "io_error"
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::DbConError(e) => write!(f, "{}", e),
            ServiceError::CrypterError(e) => write!(f, "{}", e),
            ServiceError::TokenError(e) => write!(f, "{}", e),
            ServiceError::TimeError(e) => write!(f, "{}", e),
            ServiceError::InvalidCredentials => write!(f, "Invalid email or password"),
            ServiceError::NotFound => write!(f, "Requested information is not found"),
            ServiceError::EmailIsUsed => write!(f, "Email is used"),
            ServiceError::ParsingJsonError(e) => write!(f, "{}", e),
            ServiceError::PcadParserError(e) => write!(f, "{}", e),
            ServiceError::IOError(e) => write!(f, "{}", e)
        }
    }
}

impl From<IOError> for ServiceError {
    fn from(e: IOError) -> Self {
        ServiceError::IOError(e)
    }
}

impl From<ParserError> for ServiceError {
    fn from(e: ParserError) -> Self {
        ServiceError::PcadParserError(e)
    }
}

impl From<SerdeJsonError> for ServiceError {
    fn from(e: SerdeJsonError) -> Self {
        ServiceError::ParsingJsonError(e)
    }
}

impl From<SystemTimeError> for ServiceError {
    fn from(e: SystemTimeError) -> Self {
        ServiceError::TimeError(e)
    }
}

impl From<JwtError> for ServiceError {
    fn from(e: JwtError) -> Self {
        ServiceError::TokenError(e)
    }
}

impl From<SqlxError> for ServiceError {
    fn from(e: SqlxError) -> Self {
        ServiceError::DbConError(e)
    }
}

impl From<BcryptError> for ServiceError {
    fn from(e: BcryptError) -> Self {
        ServiceError::CrypterError(e)
    }
}
