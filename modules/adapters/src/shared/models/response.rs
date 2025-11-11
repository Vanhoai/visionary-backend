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

impl<T: Serialize> HttpResponse<T> {
    pub fn new(status_code: StatusCode, message: &str, payload: T) -> Self {
        HttpResponse { status_code, message: message.to_string(), payload }
    }
}

impl<T: Serialize> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "code": "SUCCESS",
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
