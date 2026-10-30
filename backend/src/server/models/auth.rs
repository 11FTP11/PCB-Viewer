use serde::Deserialize;
use validator::{Validate, ValidationError};
use regex::Regex;
use std::sync::LazyLock;

static  RE_CHECK_STRING_ONLY: LazyLock<Regex> = LazyLock::new(
    || {
        Regex::new(r"^[a-zA-ZА-Яа-яЁё]+$").unwrap()
    }
);

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterBody {
    #[validate(required, length(min=2, max=32), custom(function="validate_string_only"))]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[validate(required, length(min=2, max=32), custom(function="validate_string_only"))]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[validate(required, length(min=8, max=128))]
    pub password: Option<String>,
    #[validate(required, email, length(min=6, max=255))]
    pub email: Option<String>
}

fn validate_string_only(val: &str) -> Result<(), ValidationError> {
    if RE_CHECK_STRING_ONLY.is_match(val) {
        return Ok(());
    } 

    Err(ValidationError::new("only_letters"))
}



#[derive(Debug, Deserialize, Validate)]
pub struct AuthBody {
    #[validate(required)]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>
}

