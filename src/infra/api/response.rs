use crate::infra::{core::result::Result, error::BoxError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::handler::handle;

pub fn from_result<T>(result: Result<T>) -> Response
where
    T: Serialize,
{
    match result {
        Ok(obj) => Json(obj).into_response(),
        Err(box_err) => from_error(box_err),
    }
}

pub fn from_error(error: BoxError) -> Response {
    let problem = handle(&error);
    let body = Json(&problem);

    (StatusCode::from_u16(problem.status).unwrap(), body).into_response()
}
