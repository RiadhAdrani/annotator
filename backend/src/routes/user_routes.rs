use actix_web::http::StatusCode;
use actix_web::{HttpMessage, HttpRequest};

use actix_web::{
    get,
    web::{self},
    Result, Scope,
};

use crate::middleware::auth_middleware::UserAuthContext;
use crate::{
    controllers::user_controller::UserController, models::user_model::User, object::error::ApiError,
};

#[get("/{id}")]
pub async fn get_user(id: web::Path<String>) -> Result<User, ApiError> {
    let user = UserController::get(id.to_string());

    if user.is_ok() {
        return Ok(user.unwrap());
    }

    Err(user.err().unwrap())
}

#[get("/me")]
pub async fn get_me(req: HttpRequest) -> Result<User, ApiError> {
    let ext = req.extensions();
    let ctx = ext.get::<UserAuthContext>();

    if ctx.as_ref().is_none() {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED));
    }

    let user = ctx.unwrap().user.clone();

    Ok(user)
}

pub fn user_routes() -> Scope {
    web::scope("/users").service(get_me).service(get_user)
}
