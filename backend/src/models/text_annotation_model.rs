use actix_web::{body::BoxBody, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextAnnotation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: String,
    pub user_id: ObjectId,
    pub tokens: Vec<Token>,
    pub labels: Vec<Label>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub start: i64,
    pub end: i64,
    /// reference a label in the labels array
    pub label: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTextAnnotationBody {
    pub content: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLabelBody {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLabelBody {
    pub name: Option<String>,
    pub color: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenBody {
    pub start: i64,
    pub end: i64,
    pub label: String,
}

impl Responder for TextAnnotation {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl Responder for Label {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl Responder for Token {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
