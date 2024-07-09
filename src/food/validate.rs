use rust_decimal::Decimal;

use crate::util::validation_error::ValidationError;

use super::model::FoodQuery;

impl FoodQuery {
    pub fn validate(
        name: &str,
        serving: &str,
        energy: i32,
        fat: Decimal,
        saturates: Decimal,
        carbohydrate: Decimal,
        sugars: Decimal,
        fibre: Decimal,
        protein: Decimal,
        salt: Decimal,
    ) -> Result<(), ValidationError> {
        let min = Decimal::from(0);
        let max = Decimal::from(1000);
        let valid_options = vec!["srv", "g", "ml"];

        let mut errors = ValidationError::new();

        errors.validate_string("name", name, Some(3), Some(100), None);

        errors.validate_choice("serving", &serving, &valid_options);

        errors.validate_number("energy", energy, Some(0), Some(10000));

        errors.validate_decimal("fat", fat, Some(min), Some(max));
        errors.validate_decimal("saturates", saturates, Some(min), Some(max));
        errors.validate_decimal("carbohydrate", carbohydrate, Some(min), Some(max));
        errors.validate_decimal("sugars", sugars, Some(min), Some(max));
        errors.validate_decimal("fibre", fibre, Some(min), Some(max));
        errors.validate_decimal("protein", protein, Some(min), Some(max));
        errors.validate_decimal("salt", salt, Some(min), Some(max));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
