use std::any::{Any, TypeId};

use super::problem::Problem;
use crate::infra::error::{AppError, BoxError};
use axum::http::StatusCode;

pub fn handle(error: &BoxError) -> Problem {
    println!("{:?}", error.type_id());
    println!("{:?}", TypeId::of::<AppError>());
    if error.is::<AppError>() {
        return Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "InternalServerError".to_owned(),
            "Internal Server Error".to_owned(),
            error.to_string(),
        );
    }

    panic!("Unrecoverable error succeed");
}
