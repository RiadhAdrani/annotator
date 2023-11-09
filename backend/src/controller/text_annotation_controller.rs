use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
};
use rocket::{http::Status, serde::json::Json};

use crate::{
    error::response::RequestError,
    helpers::colors_helpers::{get_next_valid_color, is_color_used, is_valid_color},
    middleware::auth_middleware::AuthContext,
    models::{
        common_models::DefaultResponse,
        text_annotation_model::{
            CreateLabelBody, CreateTextAnnotationBody, CreateTokenBody, Label, TextAnnotation,
            UpdateLabelBody,
        },
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

    pub fn create_label(
        auth: AuthContext,
        annotation_id: String,
        body: Json<CreateLabelBody>,
    ) -> Result<TextAnnotation, RequestError> {
        let object_id = ObjectId::from_str(annotation_id.as_str());

        if object_id.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert annotation id to object id".to_string()),
            ));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": object_id.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found".to_string()),
            ));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.user.id.unwrap() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found : Unauthorized".to_string()),
            ));
        }

        // check if we can add label
        let generated_color = get_next_valid_color(&annotation.labels);

        if generated_color.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Maximum number of labels reached".to_string()),
            ));
        }

        let mut label = Label {
            name: body.name.clone().to_owned(),
            color: "".to_string(),
            id: Some(ObjectId::new()),
        };

        let mut color: Option<String> = None;

        if body.color.is_some() {
            // check it
            let is_valid = is_valid_color(body.color.clone().unwrap());

            if !is_valid {
                return Err(RequestError::new(
                    Status::Conflict,
                    Some("Invalid color name".to_string()),
                ));
            }

            let is_used = is_color_used(body.color.clone().unwrap(), &annotation.labels);

            if is_used {
                return Err(RequestError::new(
                    Status::Conflict,
                    Some("Label color is already used".to_string()),
                ));
            }

            color = Some(body.color.clone().unwrap());
        } else {
            // generate
            color = Some(generated_color.unwrap());
        };

        if color.is_none() {
            return Err(RequestError::new(
                Status::Conflict,
                Some("Invalid color name".to_string()),
            ));
        }

        label.color = color.unwrap();

        // check if label exist with same text or color
        let name_exist = annotation
            .labels
            .iter()
            .find(|item| item.name == label.name);

        if name_exist.is_some() {
            return Err(RequestError::new(
                Status::Conflict,
                Some("Another label with the same name exists".to_string()),
            ));
        }

        // create label
        let creation_result = DB.text_annotation_collection.find_one_and_update(
            doc! {"_id": object_id.as_ref().unwrap()},
            doc! {"$push": {
              "labels": {
                "name": label.name,
                "color": label.color,
                "_id": label.id,
              }
            }},
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        );

        if creation_result.as_ref().is_err() || creation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create a new Label".to_string()),
            ));
        }

        let updated_annotation = creation_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn update_label(
        auth: AuthContext,
        annotation_id: String,
        label_id: String,
        body: Json<UpdateLabelBody>,
    ) -> Result<TextAnnotation, RequestError> {
        let annotation_oid = ObjectId::from_str(annotation_id.as_str());

        if annotation_oid.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert annotation id to object id".to_string()),
            ));
        }

        let label_oid = ObjectId::from_str(label_id.as_str());
        if label_oid.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert label id to object id".to_string()),
            ));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": annotation_oid.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found".to_string()),
            ));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.user.id.unwrap() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found : Unauthorized".to_string()),
            ));
        }

        // check if label exist
        let exists = annotation
            .labels
            .iter()
            .find(|item| item.id.unwrap() == label_oid.clone().unwrap());

        if exists.is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Target label does not exist".to_string()),
            ));
        }

        let mut update_doc = doc! {};

        if body.name.is_some() {
            // check if name already exists
            let name_exist = annotation
                .labels
                .iter()
                .find(|item| item.name == body.name.clone().unwrap());

            if name_exist.is_some() {
                return Err(RequestError::new(
                    Status::Conflict,
                    Some("Another label with the same name exists".to_string()),
                ));
            }

            // add it to doc
            update_doc.insert("labels.$.name", body.name.clone());
        }

        if body.color.is_some() {
            let color_valid = is_valid_color(body.color.clone().unwrap());

            if !color_valid {
                return Err(RequestError::new(
                    Status::Conflict,
                    Some("Invalid color name".to_string()),
                ));
            }

            // check if color already exists
            let color_used = is_color_used(body.color.clone().unwrap(), &annotation.labels);

            if color_used {
                return Err(RequestError::new(
                    Status::Conflict,
                    Some("Another label with the same color exists".to_string()),
                ));
            }

            // add it to doc
            update_doc.insert("labels.$.color", body.color.clone());
        }

        // update the label
        let update_result = DB.text_annotation_collection.find_one_and_update(
            doc! {
              "_id":annotation_oid.as_ref().unwrap(),"labels._id": label_oid.unwrap()
            },
            doc! {"$set":update_doc},
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        );

        if update_result.as_ref().is_err() || update_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to update label".to_string()),
            ));
        }

        let updated_annotation = update_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn delete_label(
        auth: AuthContext,
        annotation_id: String,
        label_id: String,
    ) -> Result<TextAnnotation, RequestError> {
        let annotation_oid = ObjectId::from_str(annotation_id.as_str());

        if annotation_oid.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert annotation id to object id".to_string()),
            ));
        }

        let label_oid = ObjectId::from_str(label_id.as_str());
        if label_oid.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert label id to object id".to_string()),
            ));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": annotation_oid.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found".to_string()),
            ));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.user.id.unwrap() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found : Unauthorized".to_string()),
            ));
        }

        // check if label exist
        let exists = annotation
            .labels
            .iter()
            .find(|item| item.id.unwrap() == label_oid.clone().unwrap());

        if exists.is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Target label does not exist".to_string()),
            ));
        }

        // delete label
        let update_result = DB.text_annotation_collection.find_one_and_update(
            doc! {
              "_id":annotation_oid.as_ref().unwrap()
            },
            doc! {
              "$pull":{
                "labels": {
                  "_id":{ "$in": [ label_oid.clone().unwrap() ] }
                },
                "tokens": {
                  "label":{ "$in":[ label_oid.clone().unwrap() ] }
                }
              }
            },
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        );

        if update_result.as_ref().is_err() || update_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some(update_result.err().unwrap().to_string()),
                // Some("Unable to update label".to_string()),
            ));
        }

        let updated_annotation = update_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn create_token(
        auth: AuthContext,
        annotation_id: String,
        body: Json<CreateTokenBody>,
    ) -> Result<TextAnnotation, RequestError> {
        let object_id = ObjectId::from_str(annotation_id.as_str());

        if object_id.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert annotation id to object id".to_string()),
            ));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": object_id.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found".to_string()),
            ));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.user.id.unwrap() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found : Unauthorized".to_string()),
            ));
        };

        // find label
        let label_oid = ObjectId::from_str(body.label.as_str());

        if label_oid.is_err() {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Unable to convert label id to object id".to_string()),
            ));
        }

        let existing_label = annotation.labels.iter().find(|l| {
            let o1 = l.to_owned().id.unwrap();

            return o1 == label_oid.clone().unwrap();
        });

        if existing_label.is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Label not found".to_string()),
            ));
        }

        let start = body.start.to_owned();
        let end = body.end.to_owned();

        // check start > end
        if end <= start {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Token (start) is superior/equal to (end)".to_string()),
            ));
        }

        if start.is_negative() {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Token (start) is negative".to_string()),
            ));
        }

        if end >= (annotation.content.len() as i64) {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Token (end) is superior to the content length".to_string()),
            ));
        }

        // check if some tokens are interlacing with this one
        let interlacing_tokens = annotation.tokens.iter().filter(|token| {
            return (start <= token.start && token.start <= end)
                || (start <= token.end && token.end <= end);
        });

        if interlacing_tokens.count() != 0 {
            return Err(RequestError::new(
                Status::Conflict,
                Some("Token is interlacing with existing one(s)".to_string()),
            ));
        }

        let doc = doc! {
          "_id": ObjectId::new(),
          "start": start,
          "end": end,
          "label": label_oid.clone().unwrap()
        };

        // create token
        // create label
        let creation_result = DB.text_annotation_collection.find_one_and_update(
            doc! {"_id": object_id.as_ref().unwrap()},
            doc! {"$push": {
              "tokens": doc
            }},
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        );

        if creation_result.as_ref().is_err() || creation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create a new Label".to_string()),
            ));
        }

        let updated_annotation = creation_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn delete_token(
        auth: AuthContext,
        annotation_id: String,
        token_id: String,
    ) -> Result<TextAnnotation, RequestError> {
        let annotation_oid = ObjectId::from_str(annotation_id.as_str());

        if annotation_oid.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert annotation id to object id".to_string()),
            ));
        }

        let token_oid = ObjectId::from_str(token_id.as_str());

        if token_oid.is_err() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to convert token id to object id".to_string()),
            ));
        }
        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": annotation_oid.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found".to_string()),
            ));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.user.id.unwrap() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Text annotation not found : Unauthorized".to_string()),
            ));
        };

        // find token
        let token = annotation
            .tokens
            .iter()
            .find(|t| t.id.clone().unwrap() == token_oid.clone().unwrap());

        if token.is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Token not found".to_string()),
            ));
        }

        // delete label
        let update_result = DB.text_annotation_collection.find_one_and_update(
            doc! {
              "_id":annotation_oid.as_ref().unwrap()
            },
            doc! {
              "$pull":{
                "tokens": {
                  "_id":{ "$in": [token_oid.unwrap()]  }
                }
              }
            },
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        );

        if update_result.as_ref().is_err() || update_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some(update_result.err().unwrap().to_string()),
            ));
        }

        let updated_annotation = update_result.unwrap().unwrap();

        Ok(updated_annotation)
    }
}
