use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

pub fn handle(error: &AppError) -> Problem {
    tracing::error!("{}", error);
    match error {
        AppError::NotFound(_) => Problem::from_kind(problem::Kind::NotFound, error.to_string()),
        AppError::Validation(_) => Problem::from_kind(problem::Kind::BadRequest, error.to_string()),
        AppError::Mongo(mongo_error) => Problem::from_kind(problem::Kind::InternalServerError, mongo_error.to_string()),
    }
}
