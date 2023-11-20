use actix_web::{
    get,
    web::{self},
    Result, Scope,
};

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

pub fn user_routes() -> Scope {
    web::scope("/users").service(get_user)
}
