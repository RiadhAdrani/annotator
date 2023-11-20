use actix_web::{
    post,
    web::{self},
    Result, Scope,
};

use crate::{
    controllers::auth_controller::AuthController,
    models::user_model::{CreateUserBody, SignInBody, UserAuthResponse},
    object::error::ApiError,
};

#[post("/sign-up")]
pub async fn sign_up(body: web::Json<CreateUserBody>) -> Result<UserAuthResponse, ApiError> {
    let res = AuthController::sign_up(body);

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    Ok(res.unwrap())
}

#[post("/sign-in")]
pub async fn sign_in(body: web::Json<SignInBody>) -> Result<UserAuthResponse, ApiError> {
    let res = AuthController::sign_in(body);

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    Ok(res.unwrap())
}

pub fn auth_routes() -> Scope {
    web::scope("/auth").service(sign_up).service(sign_in)
}
