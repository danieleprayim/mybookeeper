use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    NotFound(String),
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::ValidationError(err)
    }
}