use axum::response::IntoResponse;
use hyper::StatusCode;

use crate::{
    response::{ResponseBuilder, ResponseLink},
    AppError,
};

#[axum_macros::debug_handler]
pub async fn say_hello() -> Result<impl IntoResponse, AppError> {
    Ok(ResponseBuilder::new("Hello, world!")
        .message("Success")
        .link(ResponseLink::new("self", "/hello", "GET"))
        .json(StatusCode::OK))
}
