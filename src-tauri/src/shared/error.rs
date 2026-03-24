#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    NotFound(String),
    DatabaseError(String),
}

impl ToString for AppError {
    fn to_string(&self) -> String {
        match self {
            AppError::ValidationError(msg) => msg.clone(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::DatabaseError(msg) => msg.clone(),
        }
    }
}