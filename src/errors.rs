use serde::Serialize;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError{message:Some(msg), cause:_,error_type:_} => msg.clone(),
            AppError{message:None, cause:_,error_type:AppErrorType::NotFoundError} => String::from("the requested item was not found"),
            _ => "An unexpected error has occurred".to_string(),

        }
    }
    pub fn db_error(error: impl ToString) -> AppError {
        AppError{message:None, cause: Some(error.to_string()), error_type: AppErrorType::DbError}
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .json(AppErrorResponse{error:self.message()})
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error:String,
}