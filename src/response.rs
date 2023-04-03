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
    pub fn new(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            message: String::new(),
            links: vec![],
        }
    }

    pub fn message<S: Into<String>>(self, message: S) -> Self {
        Self {
            message: message.into(),
            ..self
        }
    }

    pub fn link(self, link: ResponseLink) -> Self {
        let mut links = self.links;
        links.push(link);
        Self { links, ..self }
    }

    pub fn error(error: AppError) -> Self {
        Self {
            data: None,
            error: Some(error),
            message: String::new(),
            links: vec![],
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

impl ResponseLink {
    pub fn new<S: Into<String>>(rel: S, href: S, method: S) -> Self {
        Self {
            rel: rel.into(),
            href: href.into(),
            method: method.into(),
        }
    }
}
