#[derive(Debug)]
pub enum Failure {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalServerError(String),
    MethodNotAllowed(String),
    UnknownFailure(String),
    Conflict(String),
    DatabaseError(String),
    ValidationError(String),
    NotImplemented(String),
    InternalError(String),
}

impl Failure {
    pub fn message(&self) -> &str {
        match self {
            Failure::BadRequest(msg) => msg,
            Failure::Unauthorized(msg) => msg,
            Failure::Forbidden(msg) => msg,
            Failure::NotFound(msg) => msg,
            Failure::InternalServerError(msg) => msg,
            Failure::MethodNotAllowed(msg) => msg,
            Failure::UnknownFailure(msg) => msg,
            Failure::Conflict(msg) => msg,
            Failure::DatabaseError(msg) => msg,
            Failure::ValidationError(msg) => msg,
            Failure::NotImplemented(msg) => msg,
            Failure::InternalError(msg) => msg,
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Failure::BadRequest(_) => "BAD_REQUEST",
            Failure::Unauthorized(_) => "UNAUTHORIZED",
            Failure::Forbidden(_) => "FORBIDDEN",
            Failure::NotFound(_) => "NOT_FOUND",
            Failure::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            Failure::MethodNotAllowed(_) => "METHOD_NOT_ALLOWED",
            Failure::UnknownFailure(_) => "UNKNOWN_FAILURE",
            Failure::Conflict(_) => "CONFLICT",
            Failure::DatabaseError(_) => "DATABASE_ERROR",
            Failure::ValidationError(_) => "VALIDATION_ERROR",
            Failure::NotImplemented(_) => "NOT_IMPLEMENTED",
            Failure::InternalError(_) => "INTERNAL_ERROR",
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Failure::BadRequest(_) => 400,
            Failure::Unauthorized(_) => 401,
            Failure::Forbidden(_) => 403,
            Failure::NotFound(_) => 404,
            Failure::InternalServerError(_) => 500,
            Failure::MethodNotAllowed(_) => 405,
            Failure::UnknownFailure(_) => 520,
            Failure::Conflict(_) => 409,
            Failure::DatabaseError(_) => 500,
            Failure::ValidationError(_) => 422,
            Failure::NotImplemented(_) => 401,
            Failure::InternalError(_) => 500,
        }
    }
}
