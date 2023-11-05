use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, State};

use crate::{
    error::response::RequestError,
    models::text_annotation_model::{CreateTextAnnotationBody, TextAnnotation},
    repository::mongodb_repos::MongoRepo,
};

pub struct TextAnnotationController {}

impl TextAnnotationController {
    pub fn create(
        db: &State<MongoRepo>,
        body: CreateTextAnnotationBody,
    ) -> Result<TextAnnotation, RequestError> {
        // TODO: get user from context/token
        let user_id = ObjectId::parse_str("ffffffffffffffffffffffff".to_string());

        if user_id.is_err() {
            // we throw user not found
            return Err(RequestError::new(
                Status::NotFound,
                Some("User not found".to_string()),
            ));
        }

        let new_doc = TextAnnotation {
            id: None,
            content: body.content,
            user_id: user_id.unwrap(),
            labels: vec![],
            tokens: vec![],
        };

        // create the text annotation
        let result = db.text_annotation_collection.insert_one(new_doc, None);

        if result.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create user".to_string()),
            ));
        }

        // find user and return it
        let id = result.unwrap().inserted_id;

        let user = db
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
}
