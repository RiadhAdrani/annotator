use redis::Commands;
use rocket::{http::Status, serde::json::Json};

use crate::{
    error::response::RequestError,
    helpers::{
        date_helpers::create_datetime_with_days_offset,
        password_helpers::{hash_password, verify_password},
        token_helpers::create_token_string,
    },
    models::user_model::{CreateUserBody, SignInBody, User, UserAuthResponse},
    repository::{mongodb_repos::DB, redis_repos::CACHE_DB},
};
use mongodb::bson::doc;

pub struct AuthController;

impl AuthController {
    pub fn sign_up(body: Json<CreateUserBody>) -> Result<UserAuthResponse, RequestError> {
        let hashed_password = hash_password(body.password.to_owned());

        if hashed_password.is_err() {
            return Err(RequestError::new(
                Status::BadRequest,
                Some("Unable to create user".to_string()),
            ));
        }

        // create doc struct
        let doc = User {
            id: None,
            email: body.email.to_owned(),
            firstname: body.firstname.to_owned(),
            lastname: body.lastname.to_owned(),
            password: hashed_password.unwrap(),
            username: body.username.to_owned(),
        };

        // check if email is used
        let user_with_email = DB
            .user_collection
            .find_one(doc! {"email": body.email.to_owned()}, None);

        if user_with_email.is_ok() && user_with_email.unwrap().is_some() {
            return Err(RequestError::new(
                Status::Conflict,
                Some("Email already used".to_string()),
            ));
        }

        // check username
        let user_with_username = DB
            .user_collection
            .find_one(doc! {"username": body.username.to_owned()}, None);

        if user_with_username.is_ok() && user_with_username.unwrap().is_some() {
            return Err(RequestError::new(
                Status::Conflict,
                Some("Username already exist".to_string()),
            ));
        }

        // create user
        let result = DB.user_collection.insert_one(doc, None);

        if result.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create user".to_string()),
            ));
        }

        let id = result.unwrap().inserted_id;

        // get user
        let user_result = DB.user_collection.find_one(doc! {"_id": id}, None);

        if user_result.as_ref().is_err() || user_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to fetch created user".to_string()),
            ));
        }

        let user = user_result.unwrap().unwrap();

        // create token for 7 days
        let exp_date = create_datetime_with_days_offset(7);
        let sub = user.id.unwrap().to_string();

        let token = create_token_string(sub, exp_date.timestamp());

        if token.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create token".to_string()),
            ));
        }

        let token_string = token.unwrap();

        // add token in redis db
        let cache_cnx = CACHE_DB.client.get_connection();

        if cache_cnx.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to connect to caching db".to_string()),
            ));
        }

        let token_created = cache_cnx.unwrap().set_ex::<String, String, String>(
            token_string.to_string(),
            token_string.to_string(),
            exp_date.timestamp() as usize,
        );

        if token_created.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to add token to caching db".to_string()),
            ));
        }

        // return token
        Ok(UserAuthResponse {
            token: token_string,
        })
    }

    pub fn sign_in(json: Json<SignInBody>) -> Result<UserAuthResponse, RequestError> {
        let login = json.login.to_owned();
        let password = json.password.to_owned();

        // find user
        let user_result = DB.user_collection.find_one(
            doc! {"$or": [{"email":login.to_string()},{"username":login.to_string()},]},
            None,
        );

        if user_result.as_ref().is_err() || user_result.as_ref().unwrap().is_none() {
            return Err(RequestError::new(
                Status::NotFound,
                Some("Unable to fetch created user".to_string()),
            ));
        }

        let user = user_result.unwrap().unwrap();

        // check if password match
        if !verify_password(password.clone(), user.password.clone()) {
            return Err(RequestError::new(
                Status::BadRequest,
                Some(format!("wrong password : {} vs {}", user.password, password).to_string()),
            ));
        }

        // create a token
        // create token for 7 days
        let exp_date = create_datetime_with_days_offset(7);
        let sub = user.id.unwrap().to_string();

        let token = create_token_string(sub, exp_date.timestamp());

        if token.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to create token".to_string()),
            ));
        }

        let token_string = token.unwrap();

        // add token in redis db
        let cache_cnx = CACHE_DB.client.get_connection();

        if cache_cnx.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to connect to caching db".to_string()),
            ));
        }

        let token_created = cache_cnx.unwrap().set_ex::<String, String, String>(
            token_string.to_string(),
            token_string.to_string(),
            exp_date.timestamp() as usize,
        );

        if token_created.is_err() {
            return Err(RequestError::new(
                Status::InternalServerError,
                Some("Unable to add token to caching db".to_string()),
            ));
        }

        // return token
        Ok(UserAuthResponse {
            token: token_string,
        })
    }
}