use crate::{
    controller::auth_controller::AuthController,
    error::response::RequestError,
    models::user_model::{CreateUserBody, SignInBody, UserAuthResponse},
};
use rocket::serde::json::Json;

#[post("/sign-up", data = "<body>")]
pub fn sign_up(body: Json<CreateUserBody>) -> Result<Json<UserAuthResponse>, Json<RequestError>> {
    println!("sign up");

    let result = AuthController::sign_up(body);

    if result.is_err() {
        return Err(Json(result.err().unwrap()));
    }

    Ok(Json(result.unwrap()))
}

#[post("/sign-in", data = "<body>")]
pub fn sign_in(body: Json<SignInBody>) -> Result<Json<UserAuthResponse>, Json<RequestError>> {
    let result = AuthController::sign_in(body);

    if result.is_err() {
        return Err(Json(result.err().unwrap()));
    }

    Ok(Json(result.unwrap()))
}
