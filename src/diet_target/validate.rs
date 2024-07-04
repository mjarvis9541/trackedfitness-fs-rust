use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::util::validation_error::ValidationError;

use super::model::{DietTarget, DietTargetGramKg};

impl DietTargetGramKg {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        let min_weight = Decimal::from(20);
        let max_weight = Decimal::from(500);
        let min_per_kg = Decimal::from(0);
        let max_per_kg = Decimal::from(10);

        errors.validate_date("date", self.date, 365, 365);

        errors.validate_decimal("weight", self.weight, Some(min_weight), Some(max_weight));
        errors.validate_decimal(
            "protein_per_kg",
            self.protein_per_kg,
            Some(min_per_kg),
            Some(max_per_kg),
        );
        errors.validate_decimal(
            "carbohydrate_per_kg",
            self.carbohydrate_per_kg,
            Some(min_per_kg),
            Some(max_per_kg),
        );
        errors.validate_decimal(
            "fat_per_kg",
            self.fat_per_kg,
            Some(min_per_kg),
            Some(max_per_kg),
        );

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl DietTarget {
    pub fn validate(
        date: NaiveDate,
        weight: Decimal,
        protein_per_kg: Decimal,
        carbohydrate_per_kg: Decimal,
        fat_per_kg: Decimal,
    ) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        let min_weight = Decimal::from(20);
        let max_weight = Decimal::from(500);
        let min_per_kg = Decimal::from(0);
        let max_per_kg = Decimal::from(10);

        errors.validate_date("date", date, 365, 365);

        errors.validate_decimal("weight", weight, Some(min_weight), Some(max_weight));
        errors.validate_decimal(
            "protein_per_kg",
            protein_per_kg,
            Some(min_per_kg),
            Some(max_per_kg),
        );
        errors.validate_decimal(
            "carbohydrate_per_kg",
            carbohydrate_per_kg,
            Some(min_per_kg),
            Some(max_per_kg),
        );
        errors.validate_decimal("fat_per_kg", fat_per_kg, Some(min_per_kg), Some(max_per_kg));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_date(date: NaiveDate) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();
        errors.validate_date("date", date, 365, 365);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
