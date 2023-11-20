use actix_web::{http::StatusCode, web::Json};
use redis::Commands;

use crate::{
    database::{mongodb::DB, redis::CACHE_DB},
    helpers::{
        date_helpers::create_datetime_with_days_offset,
        password_helpers::{hash_password, verify_password},
        token_helpers::create_token_string,
    },
    models::user_model::{CreateUserBody, SignInBody, User, UserAuthResponse},
    object::error::ApiError,
    validators::{
        types::validator::BodyValidationHelpers,
        user_validator::{CreateUserBodyValidationResult, SignInValidationResult},
    },
};
use mongodb::bson::doc;

pub struct AuthController;

impl AuthController {
    pub fn sign_up(json: Json<CreateUserBody>) -> Result<UserAuthResponse, ApiError> {
        let body = CreateUserBody {
            email: json.email.to_owned(),
            firstname: json.firstname.to_owned(),
            lastname: json.lastname.to_owned(),
            password: json.password.to_owned(),
            username: json.username.to_owned(),
        };

        let validation = CreateUserBodyValidationResult::new(&body);

        if !validation.is_valid() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("failed to validate body")
                .set_validation(validation.to_vec()));
        }

        let hashed_password = hash_password(body.password.to_owned());

        if hashed_password.is_err() {
            return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                .set_msg("unable to retrieve user")
                .set_error(hashed_password.err().unwrap().description.as_str()));
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
            return Err(ApiError::new(StatusCode::CONFLICT).set_msg("email already in use"));
        }

        // check username
        let user_with_username = DB
            .user_collection
            .find_one(doc! {"username": body.username.to_owned()}, None);

        if user_with_username.is_ok() && user_with_username.unwrap().is_some() {
            return Err(ApiError::new(StatusCode::CONFLICT).set_msg("username already in use"));
        }

        // create user
        let result = DB.user_collection.insert_one(doc, None);

        if result.is_err() {
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to create user")
            );
        }

        let id = result.unwrap().inserted_id;

        // get user
        let user_result = DB.user_collection.find_one(doc! {"_id": id}, None);

        if user_result.as_ref().is_err() || user_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to retrieve created user"));
        }

        let user = user_result.unwrap().unwrap();

        // create token for 7 days
        let exp_date = create_datetime_with_days_offset(7);
        let sub = user.id.unwrap().to_string();

        let token = create_token_string(sub, exp_date.timestamp());

        if token.is_err() {
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to create token")
            );
        }

        let token_string = token.unwrap();

        // add token in redis db
        let cache_cnx = CACHE_DB.client.get_connection();

        if cache_cnx.is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to connect to caching db"));
        }

        let token_created = cache_cnx.unwrap().set_ex::<String, String, String>(
            token_string.to_string(),
            token_string.to_string(),
            exp_date.timestamp() as usize,
        );

        if token_created.is_err() {
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to cache token")
            );
        }

        // return token
        Ok(UserAuthResponse {
            token: token_string,
        })
    }

    pub fn sign_in(json: Json<SignInBody>) -> Result<UserAuthResponse, ApiError> {
        let login = json.login.to_owned();
        let password = json.password.to_owned();

        let validation = SignInValidationResult::new(&SignInBody {
            login: login.clone(),
            password: password.clone(),
        });

        if !validation.is_valid() {
            return Err(
                ApiError::new(StatusCode::UNPROCESSABLE_ENTITY).set_msg("invalid request body")
            );
        }

        // find user
        let user_result = DB.user_collection.find_one(
            doc! {"$or": [{"email":login.clone().to_string()},{"username":login.to_string()},]},
            None,
        );

        if user_result.as_ref().is_err() || user_result.as_ref().unwrap().is_none() {
            return Err(ApiError::new(StatusCode::NOT_FOUND).set_msg("user not found"));
        }

        let user = user_result.unwrap().unwrap();

        // check if password match
        if !verify_password(password.clone(), user.password.clone()) {
            return Err(ApiError::new(StatusCode::BAD_REQUEST)
                .set_msg("login and password does not match any user"));
        }

        // create a token
        // create token for 7 days
        let exp_date = create_datetime_with_days_offset(7);
        let sub = user.id.unwrap().to_string();

        let token = create_token_string(sub, exp_date.timestamp());

        if token.is_err() {
            return Err(
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).set_msg("unable to create token")
            );
        }

        let token_string = token.unwrap();

        // add token in redis db
        let cache_cnx = CACHE_DB.client.get_connection();

        if cache_cnx.is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to connect to caching db"));
        }

        let token_created = cache_cnx.unwrap().set_ex::<String, String, String>(
            token_string.to_string(),
            token_string.to_string(),
            exp_date.timestamp() as usize,
        );

        if token_created.is_err() {
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .set_msg("unable to store token in cache"));
        }

        // return token
        Ok(UserAuthResponse {
            token: token_string,
        })
    }
}
