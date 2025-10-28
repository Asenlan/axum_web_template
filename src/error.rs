use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub status: i32,
    pub content: String,
    pub msg: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("create message error: {0}")]
    CreateMessageError(String),

    #[error("{0}")]
    ChatFileError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),

    #[error("Duckdb error : {0}")]
    DuckdbError(#[from] duckdb::Error),

    #[error("serde_json error : {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("arrow  error: {0}")]
    ArrowError(#[from] arrow::error::ArrowError),

    #[error("{0}")]
    JsonExtractorRejection(String),

    #[error("{0}")]
    TimeoutError(#[from] tower_http::timeout::TimeoutError),
    #[error("{0}")]
    AxumError(#[from] axum::Error),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            status: 500,
            content: "".to_string(),
            msg: error.into(),
        }
    }
}
impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        let error_message = rejection.body_text();
        AppError::JsonExtractorRejection(error_message)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            Self::CreateChatError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CreateMessageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ChatFileError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DuckdbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SerdeJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ArrowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::JsonExtractorRejection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TimeoutError(_) => StatusCode::REQUEST_TIMEOUT,
            Self::AxumError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        error!("Error: {:?}", Json(ErrorOutput::new(self.to_string())));
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
