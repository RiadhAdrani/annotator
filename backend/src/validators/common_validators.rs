use super::types::{string_validator::StringValidator, validator::ValidationResult};

pub struct CommonValidator;

impl CommonValidator {
    pub fn name(v: String) -> ValidationResult<String> {
        StringValidator::new(v)
            .length(2, 30)
            .is_alphanumerical()
            .no_number()
            .validate()
    }

    pub fn username(v: String) -> ValidationResult<String> {
        StringValidator::new(v)
            .length(2, 20)
            .is_alphanumerical()
            .validate()
    }

    pub fn password(v: String) -> ValidationResult<String> {
        StringValidator::new(v)
            .length(8, 100)
            .contains_lowercase()
            .contains_uppercase()
            .contains_number()
            .contains_special_char()
            .validate()
    }

    pub fn email(v: String) -> ValidationResult<String> {
        StringValidator::new(v).max_len(100).is_email().validate()
    }
}
