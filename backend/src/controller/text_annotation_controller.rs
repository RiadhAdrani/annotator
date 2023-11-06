use std::str::FromStr;

use mongodb::bson::{doc, oid::ObjectId};
use rocket::http::Status;

use crate::{
    error::response::RequestError,
    middleware::auth_middleware::AuthContext,
    models::text_annotation_model::{CreateTextAnnotationBody, TextAnnotation},
    repository::mongodb_repos::DB,
};

pub struct TextAnnotationController {}

impl TextAnnotationController {
    pub fn create(
        auth: AuthContext,
        body: CreateTextAnnotationBody,
    ) -> Result<TextAnnotation, RequestError> {
        let user_result = DB.user_collection.find_one(
            doc! {"_id": ObjectId::from_str(auth.user_id.as_str()).unwrap()},
            None,
        );

        if user_result.as_ref().is_err() || user_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("User not found".to_string()),
            ));
        }

        let user = user_result.unwrap().unwrap();

        let user_id = user.id.unwrap();

        let new_doc = TextAnnotation {
            id: None,
            content: body.content,
            user_id,
            labels: vec![],
            tokens: vec![],
        };

        // create the text annotation
        let result = DB.text_annotation_collection.insert_one(new_doc, None);

        if result.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create user".to_string()),
            ));
        }

        // find user and return it
        let id = result.unwrap().inserted_id;

        let user = DB
            .text_annotation_collection
            .find_one(doc! {"_id":id}, None);

        if user.as_ref().is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to retrieve created user".to_string()),
            ));
        }

        if user.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to retrieve created user".to_string()),
            ));
        }

        Ok(user.unwrap().unwrap())
    }

    // pub fn create_label(
    //     db: &State<MongoRepo>,
    //     body: CreateLabelBody,
    // ) -> Result<TextAnnotation, RequestError> {
    // }
}
