use pwhash::bcrypt;

use crate::object::common::CommonError;

pub fn hash_password(password: String) -> Result<String, CommonError> {
    let result = bcrypt::hash(password);

    if result.is_err() {
        return Err(CommonError {
            description: "Unable to hash password".to_string(),
        });
    }

    Ok(result.unwrap())
}

pub fn verify_password(password: String, hashed: String) -> bool {
    let is_ok = bcrypt::verify(password, hashed.as_str());

    is_ok
}
