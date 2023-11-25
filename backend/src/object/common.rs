use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonError {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    status: u16,
    status_info: String,
    msg: String,
}

impl Message {
    pub fn new() -> Message {
        Message {
            status: 200,
            status_info: "OK".to_string(),
            msg: "".to_string(),
        }
    }

    pub fn set_status(mut self, status: StatusCode) -> Message {
        self.status_info = status.to_string();
        self.status = status.as_u16();

        self
    }

    pub fn set_msg(mut self, msg: &str) -> Message {
        self.msg = msg.to_string();

        self
    }
}

impl Responder for Message {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationQueryParams {
    pub page: i64,
    pub count: i64,
}
