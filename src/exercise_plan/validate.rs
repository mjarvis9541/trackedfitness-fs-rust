use rust_decimal::Decimal;

use crate::util::validation_error::ValidationError;

use super::model::ExercisePlanInput;

impl ExercisePlanInput {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let min_weight = Decimal::from(0);
        let max_weight = Decimal::from(1000);

        let mut errors = ValidationError::new();

        errors.validate_number("sequence", self.sequence, Some(1), Some(100));
        errors.validate_decimal("weight", self.weight, Some(min_weight), Some(max_weight));

        errors.validate_number("sets", self.sets, Some(1), Some(100));
        errors.validate_number("reps", self.reps, Some(0), Some(1000));
        errors.validate_number("rest", self.rest, Some(0), Some(1000));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
