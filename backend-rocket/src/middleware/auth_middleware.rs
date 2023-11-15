use std::str::FromStr;

use redis::Commands;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};

use request::Outcome;

use crate::{
    error::response::RequestError,
    helpers::token_helpers::get_token_claims,
    models::user_model::User,
    repository::{mongodb_repos::DB, redis_repos::CACHE_DB},
};

use mongodb::bson::{doc, oid::ObjectId};

pub struct AuthContext {
    pub user_id: String,
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthContext {
    type Error = RequestError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let bearer_token = req.headers().get_one("Authorization");

        if bearer_token.is_none() {
            return Outcome::Failure((
                Status::Unauthorized,
                RequestError::new(Status::Unauthorized, Some("No token found".to_string())),
            ));
        }

        let token_string = bearer_token.unwrap().strip_prefix("Bearer ");

        if token_string.is_none() {
            return Outcome::Failure((
                Status::Unauthorized,
                RequestError::new(Status::Unauthorized, Some("No token found".to_string())),
            ));
        }

        let token = token_string.unwrap().trim().to_string();

        // find token in redis
        let redis = CACHE_DB.client.get_connection();

        if redis.is_err() {
            return Outcome::Failure((
                Status::InternalServerError,
                RequestError::new(
                    Status::InternalServerError,
                    Some("Unable to connect to caching db".to_string()),
                ),
            ));
        }

        let stored = redis.unwrap().get::<String, String>(token.to_string());

        if stored.is_err() {
            return Outcome::Failure((
                Status::Unauthorized,
                RequestError::new(Status::Unauthorized, Some("No token found".to_string())),
            ));
        }

        // extract data
        let data = get_token_claims(token.to_string());

        if data.is_err() {
            return Outcome::Failure((
                Status::Unauthorized,
                RequestError::new(Status::Unauthorized, Some("Invalid token".to_string())),
            ));
        }

        let claims = data.ok().unwrap();

        let object_id = ObjectId::from_str(claims.sub.as_str());

        let user = DB
            .user_collection
            .find_one(doc! {"_id": object_id.unwrap()}, None);

        if user.as_ref().is_err() || user.as_ref().unwrap().is_none() {
            return Outcome::Failure((
                Status::Unauthorized,
                RequestError::new(Status::Unauthorized, Some("Invalid token".to_string())),
            ));
        }

        let user_id = user
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .id
            .unwrap()
            .to_string();

        let exp = CACHE_DB
            .client
            .get_connection()
            .unwrap()
            .get::<String, String>(token.to_string());

        if exp.is_err() {
            return Outcome::Failure((
                Status::Unauthorized,
                RequestError::new(Status::Unauthorized, Some("Token expired".to_string())),
            ));
        }

        // construct an AuthContext or return an error
        let auth_context = AuthContext {
            user_id, // Replace with the actual user ID
            user: user.unwrap().unwrap(),
        };

        Outcome::Success(auth_context)
    }
}
