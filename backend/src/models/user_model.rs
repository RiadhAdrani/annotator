use actix_web::{body::BoxBody, HttpResponse, Responder};

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub username: String,
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
pub struct UpdateUserBody {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthResponse {
    pub token: String,
}

impl Responder for CreateUserBody {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(PublicUser::from(self))
    }
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        let id = match value.id {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };

        PublicUser {
            id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
            username: value.username,
        }
    }
}

impl Responder for UserAuthResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
