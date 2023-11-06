use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use serde::Serialize;

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl ErrorResponse {
    pub fn new(status_code: u16, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("DatabaseError: {0}")]
    DatabaseError(surrealdb::Error),

    #[allow(dead_code)]
    #[error("Provided Bad Request Data: Error => {0}")]
    BadRequest(&'static str),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
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
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}
