use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::modal::response::{Message, Response, Error};

pub fn response(status_code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (
        status_code,
        Json(json!(Response {
            data: Message { code: status_code.as_u16(), message: message.to_string() }
        })),
    )
}

pub fn error(status_code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (
        status_code,
        Json(json!(Error {
            error: Message { code: status_code.as_u16(), message: message.to_string() }
        })),
    )
}