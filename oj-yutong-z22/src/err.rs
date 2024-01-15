use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Error {
    pub code: u32,
    pub reason: ErrorType,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorType {
    #[serde(rename = "ERR_INVALID_ARGUMENT")]
    ErrInvalidArgument,
    #[serde(rename = "ERR_INVALID_STATE")]
    ERRINVALIDSTATE,
    #[serde(rename = "ERR_NOT_FOUND")]
    ErrNotFound,
    #[serde(rename = "ERR_RATE_LIMIT")]
    ErrRateLimit,
    #[serde(rename = "ERR_EXTERNAL")]
    ErrExternal,
    #[serde(rename = "ERR_INTERNAL")]
    ErrInternal,
}

impl Error {
    pub fn response(&self) -> HttpResponse{
        match self.reason {
            ErrorType::ErrInvalidArgument => {
                HttpResponse::BadRequest().json(self)
            }
            ErrorType::ERRINVALIDSTATE => {
                HttpResponse::BadRequest().json(self)
            }
            ErrorType::ErrNotFound => {
                HttpResponse::NotFound().json(self)
            }
            ErrorType::ErrRateLimit => {
                HttpResponse::BadRequest().json(self)
            }
            ErrorType::ErrExternal => {
                HttpResponse::InternalServerError().json(self)
            }
            ErrorType::ErrInternal => {
                HttpResponse::InternalServerError().json(self)
            }
        }
    }
}