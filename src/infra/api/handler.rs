use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

pub fn handle(error: &AppError) -> Problem {
    match error {
        AppError::NotFound(msg) => Problem::from_kind(problem::Kind::NotFound, msg.clone()),
        AppError::BadRequest(msg) => Problem::from_kind(problem::Kind::BadRequest, msg.clone()),
        AppError::Mongo(err) => Problem::from_kind(problem::Kind::InternalServerError, err.to_string()),
    }
}
