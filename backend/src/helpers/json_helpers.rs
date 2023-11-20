use actix_web::{web, Responder, Result};
use serde::Serialize;

pub fn to_json<T: Responder + Serialize>(o: T) -> Result<web::Json<T>> {
    Ok(web::Json(o))
}
