use actix_web::{dev::ServiceRequest, HttpMessage};

use mongodb::bson::oid::ObjectId;

use mongodb::bson::doc;

use crate::{
    database::mongodb::DB,
    helpers::token_helpers::{get_token_claims, get_token_from_auth_string},
    models::user_model::User,
};

#[derive(Debug, Clone)]
pub struct UserAuthContext {
    pub user: User,
    pub user_id: ObjectId,
    pub token: String,
}

pub fn use_auth_middleware(req: &ServiceRequest) {
    let authorization_result = req.headers().get("authorization");

    if authorization_result.is_none() {
        // no token found
        return;
    }

    let authorization = authorization_result.unwrap().to_str();

    if authorization.is_err() {
        return;
    }

    let bearer = authorization.unwrap();

    // check if
    let token = get_token_from_auth_string(bearer);

    if token.as_ref().is_err() {
        return;
    }

    let claim_result = get_token_claims(token.as_ref().unwrap().clone());

    if claim_result.is_err() {
        return;
    }

    let id = claim_result.unwrap().sub;

    let user_id = ObjectId::parse_str(&id);

    if user_id.as_ref().is_err() {
        return;
    }

    // find user
    let user_result = DB
        .user_collection
        .find_one(doc! {"_id":user_id.clone().unwrap()}, None);

    if user_result.as_ref().is_err() || user_result.as_ref().unwrap().is_none() {
        return;
    }

    let user = user_result.unwrap().unwrap();

    let ctx = UserAuthContext {
        token: token.unwrap(),
        user_id: user_id.unwrap(),
        user,
    };

    req.extensions_mut().insert::<UserAuthContext>(ctx);
}
