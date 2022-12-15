use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

pub fn handle(error: &AppError) -> Problem {
    match error {
        AppError::NotFound(msg) => {
            Problem::from_type(problem::Kind::NotFound, "NotFound".to_owned(), msg.clone())
        }
        _ => Problem::from_type(
            problem::Kind::InternalServerError,
            "InternalServerError".to_owned(),
            error.to_string(),
        ),
    };

    panic!("Unrecoverable error succeed");
}
