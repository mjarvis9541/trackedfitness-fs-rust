use rust_decimal::Decimal;

use crate::meal_food::model::MealFood;
use crate::util::validation_error::ValidationError;

use super::model::Meal;

impl Meal {
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

impl MealFood {
    pub fn validate(quantity: Decimal) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_decimal(
            "quantity",
            quantity,
            Some(Decimal::from(0)),
            Some(Decimal::from(1000)),
        );

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
