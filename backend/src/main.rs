mod api;
mod controller;
mod error;
mod helpers;
mod middleware;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use api::{
    auth_api::{sign_in, sign_up},
    text_annotation_api::{
        create_text_annotation, delete_text_annotation, get_text_annotation, get_text_annotations,
    },
    user_api::{create_user, get_user, update_user},
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sign_in, sign_up])
        .mount("/user", routes![create_user, get_user, update_user])
        .mount(
            "/annotations/text",
            routes![
                create_text_annotation,
                delete_text_annotation,
                get_text_annotation,
                get_text_annotations
            ],
        )
}
