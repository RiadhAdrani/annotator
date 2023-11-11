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
        create_text_annotation, create_text_annotation_label, create_text_annotation_token,
        delete_text_annotation, delete_text_annotation_label, delete_text_annotation_token,
        get_text_annotation, get_text_annotations, update_text_annotation_label,
    },
    user_api::{get_user, update_user},
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sign_in, sign_up])
        .mount("/user", routes![get_user, update_user])
        .mount(
            "/annotations/text",
            routes![
                create_text_annotation,
                delete_text_annotation,
                get_text_annotation,
                get_text_annotations,
                // labels
                create_text_annotation_label,
                update_text_annotation_label,
                delete_text_annotation_label,
                // tokens
                create_text_annotation_token,
                delete_text_annotation_token
            ],
        )
}
