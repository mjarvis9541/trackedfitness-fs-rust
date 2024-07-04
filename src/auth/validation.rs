use crate::auth::model::User;

use crate::config::get_config;
use crate::util::validation_error::ValidationError;

impl User {
    pub fn validate_signup(
        name: &str,
        email: &str,
        password: &str,
        code: &str,
    ) -> Result<(), ValidationError> {
        let config = get_config();
        let mut errors = ValidationError::new();
        if code != config.access_code {
            errors.add_non_field_error("Invalid access code");
            return Err(errors);
        }

        errors.validate_string("name", name, Some(3), Some(255), None);
        errors.validate_email("email", email);
        errors.validate_string("password", password, Some(8), Some(255), None);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_login(email: &str, password: &str) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();
        errors.validate_email("email", email);
        errors.validate_string("password", password, Some(8), Some(255), None);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();
        errors.validate_email("email", email);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_password(password: &str) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();
        errors.validate_string("password", password, Some(8), Some(255), None);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_password_change(
        old_password: &str,
        new_password: &str,
    ) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();
        if old_password == new_password {
            errors.add_non_field_error("passwords must not match");
            return Err(errors);
        }
        errors.validate_string("old_password", old_password, Some(8), Some(255), None);
        errors.validate_string("new_password", new_password, Some(8), Some(255), None);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn verify_password(&self, password: &str) -> crate::error::Result<()> {
        if bcrypt::verify(password, &self.password)? {
            Ok(())
        } else {
            Err(crate::error::Error::InvalidCredentials)
        }
    }
}
