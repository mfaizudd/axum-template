use axum::response::IntoResponse;
use hyper::StatusCode;

use crate::{response::Response, AppError};

#[axum_macros::debug_handler]
pub async fn say_hello() -> Result<impl IntoResponse, AppError> {
    Ok(Response::new("Hello, world!", "Success".to_string(), vec![]).json(StatusCode::OK))
}
