use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidationResult<T> {
    pub value: Option<T>,
    pub is_valid: bool,
    pub errors: Option<Vec<String>>,
}

impl<T> ValidationResult<T> {
    pub fn new(value: Option<T>, errors: Option<Vec<String>>) -> ValidationResult<T> {
        if value.is_none() && errors.is_none() {
            panic!("Invalid validation result")
        }

        // value is valid
        if value.is_some() {
            return ValidationResult {
                value: Some(value.unwrap()),
                is_valid: true,
                errors: None,
            };
        }

        // errors exist
        if errors.as_ref().unwrap().len() == 0 {
            panic!("No explanation for invalid value")
        }

        ValidationResult {
            value: None,
            is_valid: false,
            errors: Some(errors.unwrap()),
        }
    }

    pub fn to_vec(&self, field: &str) -> Vec<String> {
        if self.is_valid {
            return vec![];
        }

        let vec = self
            .errors
            .clone()
            .unwrap()
            .iter()
            .map(|e| format!("\"{}\": {}", field.clone(), e.clone()))
            .collect();

        vec
    }
}

pub trait BodyValidationHelpers {
    fn is_valid(&self) -> bool {
        false
    }

    fn to_vec(&self) -> Vec<String>;
}
