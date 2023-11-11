use crate::{
    controller::user_controller::UserController,
    error::response::RequestError,
    models::user_model::{UpdateUserBody, User},
};
use rocket::serde::json::Json;

#[get("/<id>")]
pub fn get_user(id: &str) -> Result<Json<User>, Json<RequestError>> {
    let data = UserController::get(id.to_string());

    if data.is_err() {
        return Err(Json(data.err().unwrap()));
    }

    Ok(Json(data.unwrap()))
}

#[put("/<id>", data = "<_body>")]
pub fn update_user(
    id: &str,
    _body: Json<UpdateUserBody>,
) -> Result<Json<User>, Json<RequestError>> {
    let body = UpdateUserBody {
        firstname: _body.firstname.to_owned(),
        lastname: _body.lastname.to_owned(),
        password: _body.lastname.to_owned(),
    };

    let data = UserController::update(id.to_string(), body);

    if data.is_err() {
        return Err(Json(data.err().unwrap()));
    }

    Ok(Json(data.unwrap()))
}
