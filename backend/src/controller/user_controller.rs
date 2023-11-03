use crate::models::user_model::User;
use crate::repository::mongodb_repos::MongoRepo;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};
use rocket::State;

pub fn user_create(repo: &State<MongoRepo>, new_user: User) -> Result<InsertOneResult, Error> {
    let new_doc = User {
        id: None,
        name: new_user.name,
        location: new_user.location,
        title: new_user.title,
    };

    let user = repo
        .user_collection
        .insert_one(new_doc, None)
        .ok()
        .expect("Error creating user");

    Ok(user)
}
