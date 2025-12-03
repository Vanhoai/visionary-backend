use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

// shared modules
use shared::models::paginate::Paginate;

pub struct HttpResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub message: String,
    pub payload: T,
}

fn map_status_to_code(status: StatusCode) -> &'static str {
    match status {
        StatusCode::OK => "Success",
        StatusCode::CREATED => "Created",
        StatusCode::BAD_REQUEST => "BadRequest",
        StatusCode::UNAUTHORIZED => "Unauthorized",
        StatusCode::FORBIDDEN => "Forbidden",
        StatusCode::NOT_FOUND => "NotFound",
        StatusCode::INTERNAL_SERVER_ERROR => "InternalServerError",
        StatusCode::METHOD_NOT_ALLOWED => "MethodNotAllowed",
        StatusCode::CONFLICT => "Conflict",
        StatusCode::NOT_IMPLEMENTED => "NotImplemented",
        _ => "UnknownStatus",
    }
}

impl<T: Serialize> HttpResponse<T> {
    pub fn new(status_code: StatusCode, message: &str, payload: T) -> Self {
        HttpResponse { status_code, message: message.to_string(), payload }
    }
}

impl<T: Serialize> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "code": map_status_to_code(self.status_code),
            "message": self.message,
            "payload": self.payload,
        }));

        (self.status_code, response).into_response()
    }
}

pub struct HttpPaginatedResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub message: String,
    pub paginate: Paginate,
    pub payload: Vec<T>,
}

impl<T: Serialize> HttpPaginatedResponse<T> {
    pub fn new(status_code: StatusCode, message: String, paginate: Paginate, payload: Vec<T>) -> Self {
        HttpPaginatedResponse { status_code, message, paginate, payload }
    }
}

impl<T: Serialize> IntoResponse for HttpPaginatedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "code": "SUCCESS",
            "message": self.message,
            "paginate": self.paginate,
            "payload": self.payload,
        }));

        (self.status_code, response).into_response()
    }
}
