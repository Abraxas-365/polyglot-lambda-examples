#[derive(Debug, Clone)]
pub enum Error {
    InternalServerError(String),
    NotFoundError(String),
    UnauthorizedError(String),
    ValidationError { field: String, message: String },
    ConflictError(String),
}

impl Error {
    pub fn internal_server_error(message: String) -> Self {
        Error::InternalServerError(message)
    }

    pub fn not_found_error(resource: String) -> Self {
        Error::NotFoundError(resource)
    }

    pub fn unauthorized_error(operation: String) -> Self {
        Error::UnauthorizedError(operation)
    }

    pub fn validation_error(field: String, message: String) -> Self {
        Error::ValidationError { field, message }
    }

    pub fn conflict_error(resource: String) -> Self {
        Error::ConflictError(resource)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerError(msg) => {
                write!(f, "Internal server error: {}", msg)
            }
            Error::NotFoundError(resource) => {
                write!(f, "{} not found", resource)
            }
            Error::UnauthorizedError(operation) => {
                write!(f, "Unauthorized: {}", operation)
            }
            Error::ValidationError { field, message } => {
                write!(f, "Validation error for field {}: {}", field, message)
            }
            Error::ConflictError(resource) => {
                write!(f, "{} already exists", resource)
            }
        }
    }
}

impl std::error::Error for Error {}
