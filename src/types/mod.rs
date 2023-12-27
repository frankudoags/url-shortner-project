use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct StoredURL {
    pub id: String,
    pub long_url: String,
    pub short_url: String,
}

pub const BASE_URL: &str = "http://localhost:4000";

pub enum AppError {
    DatabaseError(sqlx::Error),
    IoError(std::io::Error),
    UnknownError,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            AppError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
            AppError::IoError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", err),
            ),
            AppError::UnknownError => (StatusCode::INTERNAL_SERVER_ERROR, "An error has occured".to_string()),
        };

        (status_code, message).into_response()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse<T> {
    pub message: String,
    pub data: T
}

impl<T> AppResponse<T> {
    pub fn new(message: String, data: T) -> Self {
        Self { message, data }
    }
}
