use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

pub struct HttpResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub message: String,
    pub payload: T,
}

impl<T: Serialize> HttpResponse<T> {
    pub fn new(status_code: StatusCode, message: String, payload: T) -> Self {
        HttpResponse { status_code, message, payload }
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
