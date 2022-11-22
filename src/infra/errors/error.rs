use std::{error::Error, fmt};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct AppError {
    msg: String,
}

impl AppError {
    pub fn new(msg: String) -> Self {
        AppError { msg }
    }
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
