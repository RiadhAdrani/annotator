use derive_more::{Display, Error};
use std::fmt::Debug;

use actix_web::{self, body::BoxBody, http::StatusCode, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Display, Debug, Serialize, Deserialize, Error)]
#[display(
    fmt = "{{ \"status\": {}, \"msg\": {:?}, \"error\": {:?}, \"validation\": {:?} }}",
    status,
    msg,
    error,
    validation
)]
pub struct ApiError {
    pub status: u16,
    pub msg: String,
    pub error: String,
    pub validation: Vec<String>,
}

impl ApiError {
    pub fn new(_status: StatusCode) -> ApiError {
        ApiError {
            status: _status.as_u16(),
            msg: "".to_string(),
            error: "".to_string(),
            validation: vec![],
        }
    }

    pub fn set_validation(mut self, validation: Vec<String>) -> ApiError {
        self.validation = validation;

        self
    }

    pub fn set_msg(mut self, msg: &str) -> ApiError {
        self.msg = msg.to_string();

        self
    }

    pub fn set_error(mut self, error: &str) -> ApiError {
        self.error = error.to_string();

        self
    }
}

impl Responder for ApiError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::new(StatusCode::from_u16(self.status).unwrap())
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap()
    }
}

impl From<ApiError> for HttpResponse {
    fn from(value: ApiError) -> Self {
        let status = StatusCode::from_u16(value.status).unwrap();

        HttpResponse::build(status).body(value.to_string()).into()
    }
}
