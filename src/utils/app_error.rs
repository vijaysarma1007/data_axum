use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub struct AppError {
    code: StatusCode,
    message: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}
