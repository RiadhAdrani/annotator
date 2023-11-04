use crate::{
    controller::user_controller::UserController,
    models::user_model::{UpdateUserBody, User},
    repository::mongodb_repos::MongoRepo,
};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<new_user>")]
pub fn create_user(db: &State<MongoRepo>, new_user: Json<User>) -> Result<Json<User>, Status> {
    let data = User {
        id: None,
        email: new_user.email.to_owned(),
        firstname: new_user.firstname.to_owned(),
        lastname: new_user.lastname.to_owned(),
        password: new_user.password.to_owned(),
        username: new_user.username.to_owned(),
    };

    let user_detail = UserController::create(db, data);

    match user_detail {
        Ok(user) => {
            if user.is_none() {
                return Err(Status::NotFound);
            }

            return Ok(Json(user.unwrap()));
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<id>")]
pub fn get_user(id: &str, db: &State<MongoRepo>) -> Result<Json<Option<User>>, Status> {
    let data = UserController::get(db, id.to_string());

    Ok(Json(data))
}

#[put("/user/<id>", data = "<_body>")]
pub fn update_user(
    id: &str,
    db: &State<MongoRepo>,
    _body: Json<UpdateUserBody>,
) -> Result<Json<Option<User>>, Status> {
    let body = UpdateUserBody {
        email: _body.email.to_owned(),
        firstname: _body.firstname.to_owned(),
        lastname: _body.lastname.to_owned(),
        password: _body.lastname.to_owned(),
    };

    let data = UserController::update(db, id.to_string(), body);

    Ok(Json(data))
}
