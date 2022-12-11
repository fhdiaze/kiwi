use crate::infra::error::BoxError;

pub type Result<T> = std::result::Result<T, BoxError>;
