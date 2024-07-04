use crate::util::validation_error::ValidationError;

use super::model::WorkoutPlanInput;

impl WorkoutPlanInput {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_string("name", &self.name, Some(3), Some(100), None);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
