mod api;
mod controller;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::user_api::create_user;
use repository::mongodb_repos::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build().manage(db).mount("/", routes![create_user])
}
