use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserBody {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserBody {
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInBody {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthResponse {
    pub token: String,
}
