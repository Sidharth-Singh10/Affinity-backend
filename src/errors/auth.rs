use axum::http;
use http::StatusCode;

#[derive(Debug)]
pub struct _AuthError {
    pub message: String,
    pub status_code: StatusCode,
}
