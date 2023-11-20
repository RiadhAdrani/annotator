use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppData {
    pub colors: HashMap<String, String>,
}
