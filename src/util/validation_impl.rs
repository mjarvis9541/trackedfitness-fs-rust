use chrono::NaiveDate;
use rust_decimal::Decimal;

use super::validation_error::ValidationError;
use super::validation_field::*;

impl ValidationError {
    pub fn validate_string(
        &mut self,
        field_name: &str,
        value: &str,
        min_length: Option<usize>,
        max_length: Option<usize>,
        invalid_chars: Option<&[char]>,
    ) {
        if let Some(error) = validate_non_empty(value) {
            return self.add_error(field_name, error.to_string());
        }

        if let Some(error) = validate_valid_start(value) {
            self.add_error(field_name, error.to_string());
        }

        if let Some(error) = validate_valid_end(value) {
            self.add_error(field_name, error.to_string());
        }

        if let Some(error) = validate_alphanumeric(value) {
            self.add_error(field_name, error.to_string());
        }

        if let Some(min_len) = min_length {
            if let Some(error) = validate_min_length(value, min_len) {
                self.add_error(field_name, error.to_string());
            }
        }

        if let Some(max_len) = max_length {
            if let Some(error) = validate_max_length(value, max_len) {
                self.add_error(field_name, error.to_string());
            }
        }

        if let Some(chars) = invalid_chars {
            if let Some(error) = validate_no_invalid_chars(value, chars) {
                self.add_error(field_name, error.to_string());
            }
        }
    }

    pub fn validate_email(&mut self, field_name: &str, value: &str) {
        if let Some(error) = validate_non_empty(value) {
            return self.add_error(field_name, error.to_string());
        }
        if let Some(error) = validate_email(value) {
            self.add_error(field_name, error.to_string());
        }
    }

    pub fn validate_number(
        &mut self,
        field_name: &str,
        value: i32,
        min_value: Option<i32>,
        max_value: Option<i32>,
    ) {
        if let Some(max) = max_value {
            if let Some(error) = validate_max_value(value, max) {
                self.add_error(field_name, error.to_string());
            }
        }
        if let Some(min) = min_value {
            if let Some(error) = validate_min_value(value, min) {
                self.add_error(field_name, error.to_string());
            }
        }
    }

    pub fn validate_decimal(
        &mut self,
        field_name: &str,
        value: Decimal,
        min_value: Option<Decimal>,
        max_value: Option<Decimal>,
        // decimal_places: Option<usize>,
    ) {
        if let Some(min) = min_value {
            if let Some(error) = validate_min_decimal_value(value, min) {
                self.add_error(field_name, error.to_string());
            }
        }

        if let Some(max) = max_value {
            if let Some(error) = validate_max_decimal_value(value, max) {
                self.add_error(field_name, error.to_string());
            }
        }

        // if let Some(places) = decimal_places {
        //     let decimal_part_length = value.fract().to_string().len() - 2; // Subtract 2 to remove "0."
        //     if decimal_part_length > places {
        //         self.add_error(
        //             field_name,
        //             format!("value has more than {} decimal places", places),
        //         );
        //     }
        // }
    }

    pub fn validate_choice(&mut self, field_name: &str, value: &str, allowed_options: &[&str]) {
        if let Some(error) = validate_choice(value, allowed_options) {
            self.add_error(field_name, error.to_string());
        }
    }

    pub fn validate_date(
        &mut self,
        field_name: &str,
        value: NaiveDate,
        max_days_past: i64,
        max_days_future: i64,
    ) {
        if let Some(error) = validate_date_not_too_far_in_past(value, max_days_past) {
            self.add_error(field_name, error.to_string());
        }

        if let Some(error) = validate_date_not_too_far_in_future(value, max_days_future) {
            self.add_error(field_name, error.to_string());
        }
    }
}
