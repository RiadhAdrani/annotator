use actix_web::{HttpMessage, HttpRequest};

use crate::middleware::auth_middleware::UserAuthContext;

pub fn get_auth_ctx(req: &HttpRequest) -> Option<UserAuthContext> {
    let ext = req.extensions();
    ext.get::<UserAuthContext>().cloned()
}
