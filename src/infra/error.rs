use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Mongo(mongodb::error::Error)
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::NotFound(msg) => format!("NotFound: {}", msg),
            AppError::Validation(msg) => format!("Validation: {}", msg),
            AppError::Mongo(err) => format!("Mongo: {}", err)
        };
        write!(f, "{}", message)
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(cause: mongodb::error::Error) -> Self {
        AppError::Mongo(cause)
    }
}