use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Serialize)]
pub struct Response {
    success: bool,
    data: Value,
}

impl Response {
    pub fn new<T: Serialize>(data: T) -> Self {
        Self {
            success: true,
            data: serde_json::to_value(data).unwrap_or(Value::Null),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    success: bool,
    error: String,
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl error::ResponseError for ErrorMessage {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl ErrorMessage {
    pub fn new(error: String) -> Self {
        Self {
            success: false,
            error,
        }
    }
}
