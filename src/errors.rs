use axum::{response::IntoResponse, Json};
use derive_more::Display;
use hyper::StatusCode;
use jsonwebtoken::errors::ErrorKind;
use serde::{ser::SerializeStruct, Serialize};
use thiserror::Error;
use validator::ValidationError;

use crate::response::Response;

#[derive(Serialize, Error, Debug, Display)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    ValidationError(#[from] ValidationError),
    AuthorizationError(String),
    InternalError(InternalError),
}

#[derive(Debug, Display)]
pub struct InternalError(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for InternalError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl Serialize for InternalError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("InternalError", 2)?;
        state.serialize_field("type", "INTERNAL_ERROR")?;
        state.serialize_field("value", &self.0.to_string())?;
        state.end()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::RowNotFound = err {
            return AppError::NotFound("Resource not found".to_string());
        }
        AppError::InternalError(err.into())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InternalError(err.into())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.into())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => {
                AppError::AuthorizationError("Token expired".to_string())
            }
            ErrorKind::InvalidToken => AppError::AuthorizationError("Invalid token".to_string()),
            ErrorKind::InvalidIssuer => AppError::AuthorizationError("Invalid issuer".to_string()),
            ErrorKind::InvalidAudience => {
                AppError::AuthorizationError("Invalid audience".to_string())
            }
            err => {
                println!("Error: {:?}", err);
                AppError::AuthorizationError("Invalid token".to_string())
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AuthorizationError(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        tracing::error!("Error: {:?}", self);
        let body: Response<()> = Response::error(self).message("An error has occurred");
        (status, Json(body)).into_response()
    }
}
