use std::fmt::Debug;

use crate::models::user_model::{CreateUserBody, SignInBody};

use super::{
    common_validators::CommonValidator,
    types::validator::{BodyValidationHelpers, ValidationResult},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserBodyValidationResult {
    email: ValidationResult<String>,
    firstname: ValidationResult<String>,
    lastname: ValidationResult<String>,
    username: ValidationResult<String>,
    password: ValidationResult<String>,
}

impl CreateUserBodyValidationResult {
    pub fn new(body: &CreateUserBody) -> CreateUserBodyValidationResult {
        let email = CommonValidator::email(body.email.clone());
        let firstname = CommonValidator::name(body.firstname.clone());
        let lastname = CommonValidator::name(body.lastname.clone());
        let username = CommonValidator::username(body.username.clone());
        let password = CommonValidator::password(body.password.clone());

        CreateUserBodyValidationResult {
            email,
            firstname,
            lastname,
            username,
            password,
        }
    }
}

impl BodyValidationHelpers for CreateUserBodyValidationResult {
    fn is_valid(&self) -> bool {
        self.email.is_valid
            && self.firstname.is_valid
            && self.lastname.is_valid
            && self.username.is_valid
            && self.password.is_valid
    }

    fn to_vec(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];

        out.extend(self.email.to_vec("email"));
        out.extend(self.firstname.to_vec("firstname"));
        out.extend(self.lastname.to_vec("lastname"));
        out.extend(self.username.to_vec("username"));
        out.extend(self.password.to_vec("password"));

        out
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInValidationResult {
    password: ValidationResult<String>,
}

impl SignInValidationResult {
    pub fn new(body: &SignInBody) -> SignInValidationResult {
        let password = CommonValidator::password(body.password.clone());

        SignInValidationResult { password }
    }
}

impl BodyValidationHelpers for SignInValidationResult {
    fn is_valid(&self) -> bool {
        self.password.is_valid.to_owned()
    }

    fn to_vec(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];

        out.extend(self.password.to_vec("password"));

        out
    }
}
