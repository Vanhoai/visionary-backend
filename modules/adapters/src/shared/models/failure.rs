use shared::models::failure::Failure;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub struct HttpFailure {
    code: String,
    status: u16,
    message: String,
}

impl HttpFailure {
    pub fn new(failure: Failure) -> Self {
        HttpFailure {
            code: failure.code().to_string(),
            message: failure.message().to_string(),
            status: failure.status_code(),
        }
    }
}

impl IntoResponse for HttpFailure {
    fn into_response(self) -> axum::response::Response {
        let response = Json(json!({
            "code": self.code,
            "message": self.message,
        }));

        let status_code = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, response).into_response()
    }
}
