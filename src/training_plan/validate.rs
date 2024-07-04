use crate::util::validation_error::ValidationError;

use super::model::TrainingPlanInput;

impl TrainingPlanInput {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_string("name", &self.name, Some(3), Some(100), None);

        errors.validate_number("duration_weeks", self.duration_weeks, Some(1), Some(100));

        if let Some(description) = &self.description {
            errors.validate_string("description", description, Some(5), Some(1000), None);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
