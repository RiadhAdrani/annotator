use crate::error::response::RequestError;
use crate::models::user_model::{UpdateUserBody, User};
use crate::repository::mongodb_repos::DB;

use mongodb::options::{FindOneAndUpdateOptions, UpdateModifications};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOneOptions,
};
use rocket::http::Status;

pub struct UserController {}

impl UserController {
    pub fn get(id: String) -> Result<User, RequestError> {
        let id_result = ObjectId::parse_str(id);

        if id_result.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to convert user id to object id".to_string()),
            ));
        }

        let id = id_result.unwrap();

        let result = DB
            .user_collection
            .find_one(doc! {"_id":id}, FindOneOptions::default());

        if result.as_ref().is_err() || result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("User not found".to_string()),
            ));
        }

        Ok(result.unwrap().unwrap())
    }

    // pub fn get_me(db: &State<MongoRepo>, id)

    pub fn update(id: String, body: UpdateUserBody) -> Result<User, RequestError> {
        // find user
        let v = ObjectId::parse_str(id.as_str());

        if v.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to convert user id to object id".to_string()),
            ));
        }

        let _id = v.unwrap();

        let user = DB.user_collection.find_one(doc! {"_id": _id}, None);

        if user.is_err() || user.unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("User does not exist".to_string()),
            ));
        }

        let mut update = doc! {};

        // TODO: validate body
        // check value by value and insert 😑 :
        // Yeah I am bad with rust, how did you know ? :
        if body.firstname.is_some() {
            update.insert("firstname", body.firstname.unwrap());
        }

        if body.lastname.is_some() {
            update.insert("lastname", body.lastname.unwrap());
        }

        if body.password.is_some() {
            update.insert("password", body.password.unwrap());
        }

        let result = DB.user_collection.find_one_and_update(
            doc! {"_id":_id},
            UpdateModifications::Document(doc! {"$set": update}),
            FindOneAndUpdateOptions::builder()
                .return_document(mongodb::options::ReturnDocument::After)
                .build(),
        );

        if result.as_ref().is_err() || result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to update user".to_string()),
            ));
        }

        Ok(result.unwrap().unwrap())
    }
}
