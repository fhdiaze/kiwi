use std::{error::Error, fmt};

use axum::{response::IntoResponse, Json, http::StatusCode};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String)
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{}", msg)
        }
        
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg)
        };
        let body = Json(json!({"error": error_message}));

        (status, body).into_response()
    }
}