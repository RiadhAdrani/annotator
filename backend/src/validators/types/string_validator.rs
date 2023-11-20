use regex::Regex;

use super::validator::ValidationResult;

pub struct StringValidator {
    value: String,
    tests: Vec<StringValidationFn>,
}

enum StringValidationFn {
    NoNumber(String),
    NoSpace(String),
    MaxLength(String, i32),
    MinLength(String, i32),
    Length(String, i32, i32),
    IsAlphanumerical(String),
    IsEmail(String),
    IsLowercase(String),
    IsUppercase(String),
    IsCapitalized(String),
    OneOf(String, Vec<String>),
    ContainsUppercase(String),
    ContainsLowercase(String),
    ContainsNumber(String),
    ContainsSpecialChar(String),
}

impl StringValidationFn {
    fn run(&self) -> Option<String> {
        match &self {
            Self::IsCapitalized(s) => {
                if let Some(_) = s.chars().next() {
                    None
                } else {
                    Some("value is not capitalized".to_string())
                }
            }
            Self::NoNumber(s) => {
                for c in s.chars() {
                    if c.is_digit(10) {
                        return Some("value contains a number".to_string());
                    }
                }
                None
            }
            Self::IsAlphanumerical(s) => {
                for c in s.chars() {
                    if !c.is_alphanumeric() {
                        return Some("value contains a special character".to_string());
                    }
                }
                None
            }
            Self::NoSpace(s) => {
                for c in s.chars() {
                    if c.is_whitespace() {
                        return Some("value contains a white space".to_string());
                    }
                }
                None
            }
            Self::MaxLength(s, n) => {
                if (s.len() as i32) <= n.clone() {
                    None
                } else {
                    Some(format!("value exceeds max length ({})", n))
                }
            }
            Self::MinLength(s, n) => {
                if (s.len() as i32) >= n.clone() {
                    None
                } else {
                    Some(format!("value is shorter than min length ({})", n))
                }
            }
            Self::Length(s, min, max) => {
                if (s.len() as i32) >= min.clone() && (s.len() as i32) <= max.clone() {
                    None
                } else {
                    Some(format!(
                        "value length should be between ({}) and ({})",
                        min, max
                    ))
                }
            }
            Self::IsEmail(s) => {
                let regex =
                    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

                if regex.is_match(s.as_str()) {
                    None
                } else {
                    Some("value is not an email".to_string())
                }
            }
            Self::IsLowercase(s) => {
                if s.chars().all(char::is_lowercase) {
                    None
                } else {
                    Some("value should be lowercased".to_string())
                }
            }
            Self::IsUppercase(s) => {
                if s.chars().all(char::is_uppercase) {
                    None
                } else {
                    Some("value should be uppercased".to_string())
                }
            }
            Self::OneOf(s, items) => {
                if items.contains(&s) {
                    None
                } else {
                    Some(format!(
                        "value should be on of the following : {}",
                        items.clone().join(", ")
                    ))
                }
            }
            Self::ContainsUppercase(s) => {
                for c in s.chars() {
                    if c.is_uppercase() {
                        return None;
                    }
                }
                return Some("value does not have an uppercase character".to_string());
            }
            Self::ContainsLowercase(s) => {
                for c in s.chars() {
                    if c.is_lowercase() {
                        return None;
                    }
                }
                return Some("value does not have an uppercase character".to_string());
            }
            Self::ContainsNumber(s) => {
                for c in s.chars() {
                    if c.is_digit(10) {
                        return None;
                    }
                }
                return Some("value does not have a digit character".to_string());
            }
            Self::ContainsSpecialChar(s) => {
                for c in s.chars() {
                    if !c.is_alphanumeric() {
                        return None;
                    }
                }
                return Some("value does not have a special character".to_string());
            }
        }
    }
}

impl StringValidator {
    pub fn new(value: String) -> StringValidator {
        StringValidator {
            value: value.trim().to_string(),
            tests: vec![],
        }
    }

    pub fn is_capitalized(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::IsCapitalized(self.value.clone()));

        self
    }

    pub fn no_number(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::NoNumber(self.value.clone()));

        self
    }

    pub fn is_alphanumerical(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::IsAlphanumerical(self.value.clone()));

        self
    }

    pub fn no_space(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::NoSpace(self.value.clone()));

        self
    }

    pub fn max_len(&mut self, l: i32) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::MaxLength(self.value.clone(), l));

        self
    }

    pub fn min_len(&mut self, l: i32) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::MinLength(self.value.clone(), l));

        self
    }

    pub fn length(&mut self, min: i32, max: i32) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::Length(self.value.clone(), min, max));

        self
    }

    pub fn is_email(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::IsEmail(self.value.clone()));

        self
    }

    pub fn is_lowercase(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::IsLowercase(self.value.clone()));

        self
    }

    pub fn is_uppercase(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::IsUppercase(self.value.clone()));

        self
    }

    pub fn is_one_of(&mut self, items: Vec<String>) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::OneOf(self.value.clone(), items));

        self
    }

    pub fn contains_uppercase(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::ContainsUppercase(self.value.clone()));

        self
    }

    pub fn contains_lowercase(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::ContainsLowercase(self.value.clone()));

        self
    }

    pub fn contains_number(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::ContainsNumber(self.value.clone()));

        self
    }

    pub fn contains_special_char(&mut self) -> &mut StringValidator {
        self.tests
            .push(StringValidationFn::ContainsSpecialChar(self.value.clone()));

        self
    }

    pub fn validate(&mut self) -> ValidationResult<String> {
        let mut errors: Vec<String> = vec![];

        let mut result = ValidationResult::<String> {
            value: None,
            is_valid: false,
            errors: None,
        };

        self.tests.iter().for_each(|t| {
            let res = t.run();

            if res.is_some() {
                errors.push(res.unwrap().to_string())
            }
        });

        if errors.is_empty() {
            result.value = Some(self.value.clone());
            result.is_valid = true;
        } else {
            result.errors = Some(errors);
        }

        result
    }
}
