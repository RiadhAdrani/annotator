mod api;
mod controller;
mod error;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::{
    text_annotation_api::create_text_annotation,
    user_api::{create_user, get_user, update_user},
};
use repository::mongodb_repos::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![create_user, get_user, update_user])
        .mount("/annotations/text", routes![create_text_annotation])
}
