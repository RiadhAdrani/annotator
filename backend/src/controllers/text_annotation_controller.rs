use std::str::FromStr;

use actix_web::{
    http::StatusCode,
    web::{self, Json},
    Result,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
};

use crate::{
    database::mongodb::DB,
    helpers::colors_helpers::{get_next_valid_color, is_color_used, is_valid_color},
    middleware::auth_middleware::UserAuthContext,
    models::text_annotation_model::{
        CreateLabelBody, CreateTextAnnotationBody, CreateTokenBody, Label, TextAnnotation,
        UpdateLabelBody,
    },
    object::{
        common::{Message, PaginationQueryParams},
        error::ApiError,
    },
};

pub struct AnnotationController {}

impl AnnotationController {
    pub fn create(
        body: Json<CreateTextAnnotationBody>,
        auth: Option<UserAuthContext>,
    ) -> Result<TextAnnotation, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to create an annotation"));
        }

        let user_id = auth.unwrap().user_id;

        let new_doc = TextAnnotation {
            title: body.title.to_owned(),
            _id: None,
            content: body.content.to_owned(),
            user_id,
            labels: vec![],
            tokens: vec![],
        };

        // create the text annotation
        let result = DB.text_annotation_collection.insert_one(new_doc, None);

        if result.is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to create annotation")
                .set_error(result.err().unwrap().to_string().as_str()));
        }

        let id = result.unwrap().inserted_id;

        let annotation = DB
            .text_annotation_collection
            .find_one(doc! {"_id":id}, None);

        if annotation.as_ref().is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to create annotation")
                .set_error(annotation.err().unwrap().to_string().as_str()));
        }

        if annotation.as_ref().clone().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to create annotation")
                .set_error(annotation.err().unwrap().to_string().as_str()));
        }

        Ok(annotation.unwrap().unwrap())
    }

    pub fn get(id: String, auth: Option<UserAuthContext>) -> Result<TextAnnotation, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to create an annotation"));
        }

        let doc_id = ObjectId::from_str(id.as_str());

        if doc_id.is_err() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to get this annotation"));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": doc_id.unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND)
                .set_msg("you need to be signed in to get this annotation"));
        }

        let annotation = annotation_result.unwrap().unwrap();

        Ok(annotation)
    }

    pub fn delete(id: String, auth: Option<UserAuthContext>) -> Result<Message, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to delete this annotation"));
        }

        let doc_id = ObjectId::from_str(id.as_str());

        if doc_id.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert id to object id"));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": doc_id.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("annotation was not found"));
        }

        // delete the annotation
        let result = DB
            .text_annotation_collection
            .delete_one(doc! {"_id":doc_id.as_ref().unwrap()}, None);

        if result.is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to delete annotation"));
        }

        Ok(Message::new().set_msg("annotation deleted successfully"))
    }

    pub fn get_page(
        query_params: web::Query<PaginationQueryParams>,
        auth: Option<UserAuthContext>,
    ) -> Result<Vec<TextAnnotation>, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to delete this annotation"));
        }

        let user_id = auth.unwrap().user_id;

        let count = query_params.count;
        let page = query_params.page;

        let skip = (count * (page - 1)) as u64;

        let fetch_result = DB.text_annotation_collection.find(
            doc! {"user_id": user_id},
            FindOptions::builder().limit(count).skip(skip).build(),
        );

        if fetch_result.is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to fetch annotations")
                .set_error(fetch_result.is_err().to_string().as_str()));
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
        annotation_id: String,
        body: Json<CreateLabelBody>,
        auth: Option<UserAuthContext>,
    ) -> Result<TextAnnotation, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to create a label for this annotation"));
        }

        let object_id = ObjectId::from_str(annotation_id.as_str());

        if object_id.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert annotation id to object id"));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": object_id.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("annotation not found"));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.unwrap().user_id {
            return Err(
                ApiError::new(StatusCode::UNAUTHORIZED).set_msg("you cannot edit this annotation")
            );
        }

        // check if we can add label
        let generated_color = get_next_valid_color(&annotation.labels);

        if generated_color.is_err() {
            return Err(
                ApiError::new(StatusCode::FORBIDDEN).set_msg("cannot generate new color for labels, you may have reached the maximum amount of labels allowed.")
            );
        }

        let mut label = Label {
            name: body.name.clone().to_owned(),
            color: "".to_string(),
            _id: Some(ObjectId::new()),
        };

        let mut _color: Option<String> = None;

        if body.color.is_some() {
            // check it
            let is_valid = is_valid_color(body.color.clone().unwrap());

            if !is_valid {
                return Err(
                    ApiError::new(StatusCode::UNPROCESSABLE_ENTITY).set_msg("invalid label color")
                );
            }

            let is_used = is_color_used(body.color.clone().unwrap(), &annotation.labels);

            if is_used {
                return Err(
                    ApiError::new(StatusCode::CONFLICT).set_msg("label color is already used")
                );
            }

            _color = Some(body.color.clone().unwrap());
        } else {
            // generate
            _color = Some(generated_color.unwrap());
        };

        if _color.is_none() {
            return Err(
                ApiError::new(StatusCode::UNPROCESSABLE_ENTITY).set_msg("label color is missing")
            );
        }

        label.color = _color.unwrap();

        // check if label exist with same text or color
        let name_exist = annotation
            .labels
            .iter()
            .find(|item| item.name == label.name);

        if name_exist.is_some() {
            return Err(ApiError::new(StatusCode::CONFLICT).set_msg("label color already used"));
        }

        // create label
        let creation_result = DB.text_annotation_collection.find_one_and_update(
            doc! {"_id": object_id.as_ref().unwrap()},
            doc! {"$push": {
              "labels": {
                "name": label.name,
                "color": label.color,
                "_id": label._id,
              }
            }},
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        );

        if creation_result.as_ref().is_err() {
            let err = creation_result.as_ref().clone().err().unwrap().to_string();

            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to create label")
                .set_msg(err.as_str()));
        }

        if creation_result.as_ref().unwrap().is_none() {
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to create label")
            );
        }

        let updated_annotation = creation_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn update_label(
        body: Json<UpdateLabelBody>,
        auth: Option<UserAuthContext>,
        params: web::Path<(String, String)>,
    ) -> Result<TextAnnotation, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to update this annotation"));
        }

        let (annotation_id, label_id) = params.into_inner();

        let annotation_oid = ObjectId::from_str(annotation_id.as_str());

        if annotation_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert annotation id to object id"));
        }

        let label_oid = ObjectId::from_str(label_id.as_str());
        if label_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert label id to object id"));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": annotation_oid.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("annotation not found"));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.unwrap().user_id {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you cannot update this annotation at the moment"));
        }

        // check if label exist
        let exists = annotation
            .labels
            .iter()
            .find(|item| item._id.unwrap() == label_oid.clone().unwrap());

        if exists.is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("label not found"));
        }

        let mut update_doc = doc! {};

        if body.name.is_some() {
            // check if name already exists
            let name_exist = annotation
                .labels
                .iter()
                .find(|item| item.name == body.name.clone().unwrap());

            if name_exist.is_some() {
                return Err(ApiError::new(StatusCode::CONFLICT)
                    .set_msg("label with the same name already exist"));
            }

            // add it to doc
            update_doc.insert("labels.$.name", body.name.clone());
        }

        if body.color.is_some() {
            let color_valid = is_valid_color(body.color.clone().unwrap());

            if !color_valid {
                return Err(ApiError::new(StatusCode::CONFLICT).set_msg("label color is invalid"));
            }

            // check if color already exists
            let color_used = is_color_used(body.color.clone().unwrap(), &annotation.labels);

            if color_used {
                return Err(ApiError::new(StatusCode::CONFLICT)
                    .set_msg("label with the same color already exist"));
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
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to create label")
            );
        }

        let updated_annotation = update_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn delete_label(
        auth: Option<UserAuthContext>,
        params: web::Path<(String, String)>,
    ) -> Result<TextAnnotation, ApiError> {
        let (annotation_id, label_id) = params.into_inner();

        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to update this annotation"));
        }

        let annotation_oid = ObjectId::from_str(annotation_id.as_str());

        if annotation_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert annotation id to object id"));
        }

        let label_oid = ObjectId::from_str(label_id.as_str());
        if label_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert label id to object id"));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": annotation_oid.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("annotation not found"));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.unwrap().user_id {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you cannot delete this annotation at the moment"));
        }

        // check if label exist
        let exists = annotation
            .labels
            .iter()
            .find(|item| item._id.unwrap() == label_oid.clone().unwrap());

        if exists.is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("label not found"));
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
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to delete label")
            );
        }

        let updated_annotation = update_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn create_token(
        annotation_id: String,
        body: Json<CreateTokenBody>,
        auth: Option<UserAuthContext>,
    ) -> Result<TextAnnotation, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to update this annotation"));
        }

        let object_id = ObjectId::from_str(annotation_id.as_str());

        if object_id.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert annotation id to object id"));
        }

        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": object_id.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("annotation not found"));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.unwrap().user_id {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you cannot update this annotation at the moment"));
        };

        // find label
        let label_oid = ObjectId::from_str(body.label.as_str());

        if label_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert label id to object id"));
        }

        let existing_label = annotation.labels.iter().find(|l| {
            let o1 = l.to_owned()._id.unwrap();

            return o1 == label_oid.clone().unwrap();
        });

        if existing_label.is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("label not found"));
        }

        let start = body.start.to_owned();
        let end = body.end.to_owned();

        // check start > end
        if end <= start {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("token start's index cannot be greater than the end's"));
        }

        if start.is_negative() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("token start cannot be negative"));
        }

        if end >= (annotation.content.len() as i64) {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("end token cannot be superior to the length of the content"));
        }

        // check if some tokens are interlacing with this one
        let interlacing_tokens = annotation.tokens.iter().filter(|token| {
            return (start <= token.start && token.start <= end)
                || (start <= token.end && token.end <= end);
        });

        if interlacing_tokens.count() != 0 {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("token is interlacing with an existing one"));
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
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to create annotation token"));
        }

        let updated_annotation = creation_result.unwrap().unwrap();

        Ok(updated_annotation)
    }

    pub fn delete_token(
        annotation_id: String,
        token_id: String,
        auth: Option<UserAuthContext>,
    ) -> Result<TextAnnotation, ApiError> {
        if auth.is_none() {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you need to be signed in to update this annotation"));
        }

        let annotation_oid = ObjectId::from_str(annotation_id.as_str());

        if annotation_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert annotation id to object id"));
        }

        let token_oid = ObjectId::from_str(token_id.as_str());

        if token_oid.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to convert token id to object id"));
        }
        // find annotation
        let annotation_result = DB
            .text_annotation_collection
            .find_one(doc! {"_id": annotation_oid.as_ref().unwrap()}, None);

        if annotation_result.as_ref().is_err() || annotation_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("annotation not found"));
        }

        let annotation = annotation_result.unwrap().unwrap();

        // check if owned by user
        if annotation.user_id != auth.unwrap().user_id {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED)
                .set_msg("you cannot update annotation at the moment"));
        };

        // find token
        let token = annotation
            .tokens
            .iter()
            .find(|t| t._id.clone().unwrap() == token_oid.clone().unwrap());

        if token.is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("token not found"));
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
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to delete token")
            );
        }

        let updated_annotation = update_result.unwrap().unwrap();

        Ok(updated_annotation)
    }
}
