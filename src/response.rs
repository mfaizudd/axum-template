use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;

use crate::AppError;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<AppError>,
    pub message: String,
    pub links: Vec<ResponseLink>,
}

impl<T: Serialize> Response<T> {
    pub fn new(data: T, message: String, links: Vec<ResponseLink>) -> Self {
        Self {
            data: Some(data),
            error: None,
            message,
            links,
        }
    }

    pub fn error(error: AppError, message: String, links: Vec<ResponseLink>) -> Self {
        Self {
            data: None,
            error: Some(error),
            message,
            links,
        }
    }

    pub fn json(&self, code: StatusCode) -> impl IntoResponse {
        (code, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct ResponseLink {
    pub rel: String,
    pub href: String,
    pub method: String,
}
