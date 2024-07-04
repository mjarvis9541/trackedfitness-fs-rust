use crate::util::validation_error::ValidationError;
use rust_decimal::Decimal;

use super::model::SetModel;

impl SetModel {
    pub fn validate(
        order: i32,
        weight: Decimal,
        reps: i32,
        rest: i32,
    ) -> Result<(), ValidationError> {
        let min_weight = Decimal::from(0);
        let max_weight = Decimal::from(1000);

        let mut errors = ValidationError::new();

        errors.validate_decimal("weight", weight, Some(min_weight), Some(max_weight));

        errors.validate_number("order", order, Some(0), Some(100));
        errors.validate_number("reps", reps, Some(0), Some(100));
        errors.validate_number("rest", rest, Some(0), Some(1000));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
