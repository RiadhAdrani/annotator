use actix_web::http::StatusCode;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOneOptions,
};

use crate::{database::mongodb::DB, models::user_model::User, object::error::ApiError};

pub struct UserController;

impl UserController {
    pub fn get(id: String) -> Result<User, ApiError> {
        let id_result = ObjectId::parse_str(id);

        if id_result.is_err() {
            return Err(
                ApiError::new(StatusCode::BAD_REQUEST).set_msg("unable to convert id to object id")
            );
        }

        let id = id_result.unwrap();

        let result = DB
            .user_collection
            .find_one(doc! {"_id":id}, FindOneOptions::default());

        if result.as_ref().is_err() || result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("user not found"));
        }

        Ok(result.unwrap().unwrap())
    }
}
