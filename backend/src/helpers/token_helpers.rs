use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::common::CommonError;

static TOKEN_SECRET: &str = "annotator_secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: i64,
}

pub fn create_token_string(subject: String, expiration_s: i64) -> Result<String, CommonError> {
    let token = encode(
        &Header::default(),
        &TokenClaims {
            sub: subject,
            exp: expiration_s,
        },
        &EncodingKey::from_secret(TOKEN_SECRET.as_ref()),
    );

    if token.is_err() {
        return Err(CommonError {
            description: format!(
                "Unable to create token - {}",
                token.err().unwrap().to_string()
            ),
        });
    }

    Ok(token.unwrap())
}

pub fn get_token_claims(token: String) -> Result<TokenClaims, CommonError> {
    let decoded = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
        &Validation::default(),
    );

    if decoded.is_err() {
        return Err(CommonError {
            description: format!(
                "Unable to decode token: {}",
                decoded.err().unwrap().to_string()
            ),
        });
    }

    Ok(decoded.unwrap().claims)
}
