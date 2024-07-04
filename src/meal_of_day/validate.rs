use crate::util::validation_error::ValidationError;

use super::model::MealOfDay;

impl MealOfDay {
    pub fn validate(name: &str, ordering: i32) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_string("name", name, Some(3), Some(100), None);

        errors.validate_number("ordering", ordering, Some(1), Some(100));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
