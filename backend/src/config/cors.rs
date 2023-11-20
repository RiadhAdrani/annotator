use actix_cors::Cors;
use actix_web::http;

use super::env::APP_URL;

pub fn create_cors() -> Cors {
    Cors::default()
        .allowed_origin(APP_URL.as_str())
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "OPTIONS"])
        .allowed_header(http::header::AUTHORIZATION)
        .allowed_header(http::header::ACCEPT)
        .allowed_header(http::header::CONTENT_TYPE)
        .allowed_header(http::header::ACCESS_CONTROL_ALLOW_HEADERS)
        .allowed_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
        .max_age(3600)
}
