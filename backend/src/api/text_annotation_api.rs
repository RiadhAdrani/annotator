use crate::{
    controller::text_annotation_controller::TextAnnotationController,
    error::response::RequestError,
    middleware::auth_middleware::AuthContext,
    models::{
        common_models::DefaultResponse,
        text_annotation_model::{CreateTextAnnotationBody, TextAnnotation},
    },
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

#[get("/<id>")]
pub fn get_text_annotation(
    auth: AuthContext,
    id: &str,
) -> Result<Json<TextAnnotation>, Json<RequestError>> {
    let response = TextAnnotationController::get(auth, id.to_string());

    if response.is_err() {
        return Err(Json(response.err().unwrap()));
    }

    Ok(Json(response.ok().unwrap()))
}

#[delete("/<id>")]
pub fn delete_text_annotation(
    auth: AuthContext,
    id: &str,
) -> Result<Json<DefaultResponse>, Json<RequestError>> {
    let response = TextAnnotationController::delete(auth, id.to_string());

    if response.is_err() {
        return Err(Json(response.err().unwrap()));
    }

    Ok(Json(response.ok().unwrap()))
}

#[get("/?<page>&<count>")]
pub fn get_text_annotations(
    auth: AuthContext,
    page: i64,
    count: i64,
) -> Result<Json<Vec<TextAnnotation>>, Json<RequestError>> {
    let response = TextAnnotationController::get_page(auth, page, count);

    if response.is_err() {
        return Err(Json(response.err().unwrap()));
    }

    Ok(Json(response.ok().unwrap()))
}
