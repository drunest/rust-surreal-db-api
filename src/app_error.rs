use actix_identity::error::GetIdentityError;
use actix_session::SessionInsertError;
use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    status_code: u16,
    error: String,
}

impl ErrorResponse {
    pub fn new(status_code: u16, message: String) -> Self {
        Self {
            status_code,
            error: message,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("DatabaseError: {0}")]
    DatabaseError(surrealdb::Error),

    #[allow(dead_code)]
    #[error("{0}")]
    BadRequest(String),

    #[error("IOError: {0}")]
    IOError(std::io::Error),

    #[allow(dead_code)]
    #[error("InternalServerError: {0}")]
    InternalError(String),

    #[allow(dead_code)]
    #[error("Unauthorized")]
    Unauthorized,

    #[allow(dead_code)]
    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[allow(dead_code)]
    #[error("UNAUTHORIZED: {0}")]
    IdentityError(GetIdentityError),

    #[allow(dead_code)]
    #[error("Error Creating Session: {0}")]
    SessionInsertError(SessionInsertError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code().as_u16();
        let error_message = self.to_string();
        let error_response = ErrorResponse::new(status_code, error_message);

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_response)
    }
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::IdentityError(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::SessionInsertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        AppError::DatabaseError(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IOError(value)
    }
}

impl From<GetIdentityError> for AppError {
    fn from(value: GetIdentityError) -> Self {
        AppError::IdentityError(value)
    }
}

impl From<SessionInsertError> for AppError {
    fn from(value: SessionInsertError) -> Self {
        AppError::SessionInsertError(value)
    }
}
