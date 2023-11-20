use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContent {
    pub code: u16,
    pub reason: String,
    pub description: Option<String>,
}
