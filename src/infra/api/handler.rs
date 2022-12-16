use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

pub fn handle(error: &AppError) -> Problem {
    match error {
        AppError::NotFound(msg) => {
            Problem::from_kind(problem::Kind::NotFound, msg.clone())
        }
        _ => Problem::from_kind(
            problem::Kind::InternalServerError,
            error.to_string(),
        ),
    };

    panic!("Unrecoverable error succeed");
}
