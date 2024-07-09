use crate::util::validation_error::ValidationError;

use super::model::Movement;

impl Movement {
    pub fn validate(name: &str) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_string("name", name, Some(3), Some(100), None);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
