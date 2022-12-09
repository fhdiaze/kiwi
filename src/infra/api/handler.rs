use axum::http::StatusCode;

use crate::infra::error::BoxError;

use super::problem::Problem;

pub fn handle(error: &BoxError) -> Problem {
    Problem::new(
        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "InternalServerError".to_owned(),
        "Internal Server Error".to_owned(),
        error.to_string(),
    )
}
