use actix_web::http::StatusCode;
use crate::utils::api_response::ApiResponse;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
    pub status_code: u16,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            status_code: 500, 
        }
    }

    pub fn with_status(message: &str, status_code: u16) -> Self {
        Self {
            message: message.to_string(),
            status_code,
        }
    }

    pub fn response(&self) -> ApiResponse {
        ApiResponse::new(self.status_code, self.message.clone())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
