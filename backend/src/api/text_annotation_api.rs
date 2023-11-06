use crate::{
    controller::text_annotation_controller::TextAnnotationController,
    error::response::RequestError,
    middleware::auth_middleware::AuthContext,
    models::text_annotation_model::{CreateTextAnnotationBody, TextAnnotation},
};
use rocket::serde::json::Json;

#[post("/", data = "<_body>")]
pub fn create_text_annotation(
    auth: AuthContext,
    _body: Json<CreateTextAnnotationBody>,
) -> Result<Json<TextAnnotation>, Json<RequestError>> {
    let body = CreateTextAnnotationBody {
        content: _body.content.to_owned(),
    };

    let response = TextAnnotationController::create(auth, body);

    if response.is_err() {
        return Err(Json(response.err().unwrap()));
    }

    Ok(Json(response.unwrap()))
}
