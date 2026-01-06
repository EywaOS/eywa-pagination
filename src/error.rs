//! # Pagination Errors
//!
//! Domain-specific errors for pagination logic.

use axum::http::StatusCode;
use eywa_errors::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaginationError {
    #[error("Invalid page number: {0}. Must be greater than 0.")]
    InvalidPage(u32),

    #[error("Invalid limit: {0}. Must be between 1 and 100.")]
    InvalidLimit(u32),

    #[error("Calculation overflow")]
    Overflow,

    #[error("Invalid pagination parameters: {0}")]
    InvalidParams(String),
}

impl From<PaginationError> for AppError {
    fn from(err: PaginationError) -> Self {
        match err {
            // These are client errors (Bad Request)
            PaginationError::InvalidPage(_)
            | PaginationError::InvalidLimit(_)
            | PaginationError::InvalidParams(_) => AppError::ValidationField {
                field: "pagination".to_string(),
                message: err.to_string(),
            },

            // This is likely an internal calculation error
            PaginationError::Overflow => AppError::InternalServerError(err.to_string()),
        }
    }
}

impl PaginationError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidPage(_) | Self::InvalidLimit(_) | Self::InvalidParams(_) => {
                StatusCode::BAD_REQUEST
            }
            Self::Overflow => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
