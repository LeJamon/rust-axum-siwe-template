use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::NotFound => write!(f, "Not found"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::DatabaseError(_) | AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
        };
        (status, body).into_response()
    }
}
