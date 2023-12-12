use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpRequest, Result, Scope,
};

use crate::{
    controllers::text_annotation_controller::AnnotationController,
    helpers::request_helpers::get_auth_ctx,
    models::text_annotation_model::{
        CreateLabelBody, CreateTextAnnotationBody, CreateTokenBody, TextAnnotation,
        UpdateLabelBody, UpdateTextAnnotationBody,
    },
    object::{
        common::{Message, PaginationQueryParams},
        error::ApiError,
    },
};

#[post("/")]
async fn create_annotation(
    body: web::Json<CreateTextAnnotationBody>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::create(body, auth);

    res
}

#[put("/{id}")]
async fn update_annotation(
    body: web::Json<UpdateTextAnnotationBody>,
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::update(id.clone(), auth, body);

    res
}

#[post("/{id}/labels")]
async fn create_label(
    body: web::Json<CreateLabelBody>,
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::create_label(id.clone(), body, auth);

    res
}

#[put("/{id}/labels/{label_id}")]
async fn update_label(
    body: web::Json<UpdateLabelBody>,
    params: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::update_label(body, auth, params);

    res
}

#[delete("/{id}/labels/{label_id}")]
async fn delete_label(
    params: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::delete_label(auth, params);

    res
}

#[post("/{id}/tokens")]
async fn create_token(
    body: web::Json<CreateTokenBody>,
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::create_token(id.clone(), body, auth);

    res
}

#[delete("/{id}/tokens/{token_id}")]
async fn delete_token(
    params: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::delete_token(params, auth);

    res
}

#[get("/{id}")]
async fn get_annotation(
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<TextAnnotation, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::get(id.to_string(), auth);

    res
}

#[get("/")]
async fn get_annotations_page(
    query_params: web::Query<PaginationQueryParams>,
    req: HttpRequest,
) -> Result<Json<Vec<TextAnnotation>>, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::get_page(query_params, auth);

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    Ok(Json(res.unwrap()))
}

#[delete("/{id}")]
async fn delete_annotation(id: web::Path<String>, req: HttpRequest) -> Result<Message, ApiError> {
    let auth = get_auth_ctx(&req);

    let res = AnnotationController::delete(id.to_string(), auth);

    res
}

pub fn annotation_routes() -> Scope {
    web::scope("/annotations/text")
        .service(create_annotation)
        .service(update_annotation)
        .service(get_annotation)
        .service(get_annotations_page)
        .service(delete_annotation)
        // labels
        .service(create_label)
        .service(update_label)
        .service(delete_label)
        // tokens
        .service(create_token)
        .service(delete_token)
}
