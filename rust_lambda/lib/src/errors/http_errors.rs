use super::errors::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HTTPError {
    pub code: u16,
    pub message: String,
}

impl HTTPError {
    pub fn new(error: Error) -> Self {
        match error {
            Error::InternalServerError(msg) => HTTPError {
                code: 500,
                message: format!("Internal server error: {}", msg),
            },
            Error::NotFoundError(resource) => HTTPError {
                code: 404,
                message: format!("{} not found", resource),
            },
            Error::UnauthorizedError(operation) => HTTPError {
                code: 401,
                message: format!("Unauthorized: {}", operation),
            },
            Error::ValidationError { field, message } => HTTPError {
                code: 400,
                message: format!("Validation error for field {}: {}", field, message),
            },
            Error::ConflictError(resource) => HTTPError {
                code: 409,
                message: format!("{} already exists", resource),
            },
        }
    }
}
