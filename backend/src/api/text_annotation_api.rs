use crate::{
    controller::text_annotation_controller::TextAnnotationController,
    error::response::RequestError,
    models::text_annotation_model::{CreateTextAnnotationBody, TextAnnotation},
    repository::mongodb_repos::MongoRepo,
};
use rocket::{serde::json::Json, State};

#[post("/", data = "<_body>")]
pub fn create_text_annotation(
    db: &State<MongoRepo>,
    _body: Json<CreateTextAnnotationBody>,
) -> Result<Json<TextAnnotation>, Json<RequestError>> {
    let body = CreateTextAnnotationBody {
        content: _body.content.to_owned(),
    };

    let response = TextAnnotationController::create(db, body);

    if response.is_err() {
        return Err(Json(response.err().unwrap()));
    }

    Ok(Json(response.unwrap()))
}
