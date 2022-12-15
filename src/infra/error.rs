use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    Mongo(mongodb::error::Error)
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "An error ocurred".to_owned())
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(cause: mongodb::error::Error) -> Self {
        AppError::Mongo(cause)
    }
}