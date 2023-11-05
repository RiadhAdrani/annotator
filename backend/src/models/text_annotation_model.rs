use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TextAnnotation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: String,
    pub user_id: ObjectId,
    pub tokens: Vec<Token>,
    pub labels: Vec<Label>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub start: i128,
    pub end: i128,
    /// reference a label in the labels array
    pub label: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTextAnnotationBody {
    pub content: String,
}
