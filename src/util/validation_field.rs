use chrono::prelude::*;
use chrono::TimeDelta;
use derive_more::Display;
use rust_decimal::Decimal;

use crate::config::get_config;

#[derive(Debug, Display)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FieldError {
    #[display(fmt = "This field is required.")]
    Required,
    #[display(fmt = "This must be a valid email address.")]
    InvalidEmail,
    #[display(fmt = "This field cannot contain any invalid characters.")]
    InvalidCharacter,
    #[display(fmt = "Date must not be too far in the past.")]
    DateTooFarInPast,
    #[display(fmt = "Date must not be too far in the future.")]
    DateTooFarInFuture,
    #[display(fmt = "Invalid option.")]
    InvalidOption,
    #[display(fmt = "Ensure this field has at least {} {}.", _0, _1)]
    MinLength(usize, String),
    #[display(fmt = "Ensure this field has no more than {} {}.", _0, _1)]
    MaxLength(usize, String),
    #[display(fmt = "Ensure this value is greater than or equal to {}.", _0)]
    MinValue(i32),
    #[display(fmt = "Ensure this value is less than or equal to {}.", _0)]
    MaxValue(i32),
    #[display(fmt = "Ensure this value is greater than or equal to {}.", _0)]
    MinDecimal(Decimal),
    #[display(fmt = "Ensure this value is less than or equal to {}.", _0)]
    MaxDecimal(Decimal),
    #[display(fmt = "This field cannot start with a special character.")]
    InvalidStart,
    #[display(fmt = "This field cannot end with a special character.")]
    InvalidEnd,
}

const ALLOWED_CHARACTERS: [char; 8] = ['/', '\'', ',', '-', '.', '"', '(', ')'];

pub fn validate_alphanumeric(value: &str) -> Option<FieldError> {
    if value
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || ALLOWED_CHARACTERS.contains(&c))
    {
        None
    } else {
        Some(FieldError::InvalidCharacter)
    }
}

pub fn validate_valid_start(value: &str) -> Option<FieldError> {
    if value.starts_with(ALLOWED_CHARACTERS) {
        Some(FieldError::InvalidStart)
    } else {
        None
    }
}

pub fn validate_valid_end(value: &str) -> Option<FieldError> {
    if value.ends_with(ALLOWED_CHARACTERS) {
        Some(FieldError::InvalidEnd)
    } else {
        None
    }
}

pub fn validate_max_length(value: &str, max_length: usize) -> Option<FieldError> {
    if value.len() > max_length {
        Some(FieldError::MaxLength(max_length, "characters".to_string()))
    } else {
        None
    }
}

pub fn validate_min_length(value: &str, min_length: usize) -> Option<FieldError> {
    if value.len() < min_length {
        Some(FieldError::MinLength(min_length, "characters".to_string()))
    } else {
        None
    }
}

pub fn validate_non_empty(value: &str) -> Option<FieldError> {
    if value.trim().is_empty() {
        Some(FieldError::Required)
    } else {
        None
    }
}

pub fn validate_no_invalid_chars(value: &str, invalid_chars: &[char]) -> Option<FieldError> {
    if invalid_chars.iter().any(|&ch| value.contains(ch)) {
        Some(FieldError::InvalidCharacter)
    } else {
        None
    }
}

// pub fn validate_no_whitespace(value: &str) -> Option<FieldError> {
//     if value.chars().any(char::is_whitespace) {
//         Some(FieldError::ContainsWhitespace)
//     } else {
//         None
//     }
// }

pub fn validate_max_value(value: i32, max_value: i32) -> Option<FieldError> {
    if value > max_value {
        Some(FieldError::MaxValue(max_value))
    } else {
        None
    }
}

pub fn validate_min_value(value: i32, min_value: i32) -> Option<FieldError> {
    if value < min_value {
        Some(FieldError::MinValue(min_value))
    } else {
        None
    }
}

pub fn validate_max_decimal_value(value: Decimal, max_value: Decimal) -> Option<FieldError> {
    if value > max_value {
        Some(FieldError::MaxDecimal(max_value))
    } else {
        None
    }
}

pub fn validate_min_decimal_value(value: Decimal, min_value: Decimal) -> Option<FieldError> {
    if value < min_value {
        Some(FieldError::MinDecimal(min_value))
    } else {
        None
    }
}

pub fn validate_date_not_too_far_in_past(
    value: NaiveDate,
    threshold_years: i64,
) -> Option<FieldError> {
    let threshold_date = Utc::now()
        .checked_sub_signed(TimeDelta::days(threshold_years * 365))
        .expect("valid date")
        .date_naive();
    if value < threshold_date {
        Some(FieldError::DateTooFarInPast)
    } else {
        None
    }
}

pub fn validate_date_not_too_far_in_future(
    value: NaiveDate,
    threshold_years: i64,
) -> Option<FieldError> {
    let threshold_date = Utc::now()
        .checked_add_signed(TimeDelta::days(threshold_years * 365))
        .expect("valid date")
        .date_naive();
    if value > threshold_date {
        Some(FieldError::DateTooFarInFuture)
    } else {
        None
    }
}

pub fn validate_choice(value: &str, allowed_options: &[&str]) -> Option<FieldError> {
    if !allowed_options.contains(&value) {
        Some(FieldError::InvalidOption)
    } else {
        None
    }
}

pub fn validate_email(value: &str) -> Option<FieldError> {
    let config = get_config();
    if config.email_regex.is_match(value) {
        None
    } else {
        Some(FieldError::InvalidEmail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert_eq!(validate_email("valid@example.com"), None);
        assert_eq!(
            validate_email("invalid-email"),
            Some(FieldError::InvalidEmail)
        );
    }
}
