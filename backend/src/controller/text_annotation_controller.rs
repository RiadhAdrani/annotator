use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use rocket::http::Status;

use crate::{
    error::response::RequestError,
    middleware::auth_middleware::AuthContext,
    models::{
        common_models::DefaultResponse,
        text_annotation_model::{CreateTextAnnotationBody, TextAnnotation},
    },
    repository::mongodb_repos::DB,
};

pub struct TextAnnotationController {}

impl TextAnnotationController {
    pub fn create(
        auth: AuthContext,
        body: CreateTextAnnotationBody,
    ) -> Result<TextAnnotation, RequestError> {
        let user_id = auth.user.id.unwrap();

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

    pub fn get(_: AuthContext, id: String) -> Result<TextAnnotation, RequestError> {
        let doc_id = ObjectId::from_str(id.as_str());

        if doc_id.is_err() {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Unable to convert id to ObjectId".to_string()),
            ));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": doc_id.unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text Annotation not found".to_string()),
            ));
        }

        let annotation = annotation_result.unwrap().unwrap();

        Ok(annotation)
    }

    pub fn delete(_: AuthContext, id: String) -> Result<DefaultResponse, RequestError> {
        let doc_id = ObjectId::from_str(id.as_str());

        if doc_id.is_err() {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Unable to convert id to ObjectId".to_string()),
            ));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": doc_id.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text Annotation not found".to_string()),
            ));
        }

        // delete the annotation
        let result = DB
            .text_annotation_collection
            .delete_one(doc! {"_id":doc_id.as_ref().unwrap()}, None);

        if result.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to delete the text annotation ".to_string()),
            ));
        }

        Ok(DefaultResponse {
            message: "Text Annotation deleted successfully".to_string(),
        })
    }

    pub fn get_page(
        auth: AuthContext,
        page: i64,
        count: i64,
    ) -> Result<Vec<TextAnnotation>, RequestError> {
        let user_id = auth.user.id.unwrap();

        let skip = (count * (page - 1)) as u64;

        let fetch_result = DB.text_annotation_collection.find(
            doc! {"user_id": user_id},
            FindOptions::builder().limit(count).skip(skip).build(),
        );

        if fetch_result.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to fetch the text annotation ".to_string()),
            ));
        }

        let mut items: Vec<TextAnnotation> = vec![];

        fetch_result.unwrap().for_each(|item| {
            if item.is_ok() {
                items.push(item.unwrap());
            }
        });

        Ok(items)
    }
}
