use rocket::http;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContent {
    pub code: u16,
    pub reason: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestError {
    pub error: ErrorContent,
}

impl RequestError {
    pub fn new(status: http::Status, description: Option<String>) -> RequestError {
        RequestError {
            error: ErrorContent {
                code: status.code,
                description,
                reason: status.reason_lossy().to_string(),
            },
        }
    }
}
