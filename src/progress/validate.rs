use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::util::validation_error::ValidationError;

use super::model::Progress;

impl Progress {
    pub fn validate(
        date: NaiveDate,
        weight: Option<Decimal>,
        energy_burnt: Option<i32>,
        notes: Option<String>,
    ) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_date("date", date, 365, 10);

        if let Some(weight) = weight {
            let min_weight = Decimal::from(20);
            let max_weight = Decimal::from(500);
            errors.validate_decimal("weight", weight, Some(min_weight), Some(max_weight));
        }

        if let Some(energy) = energy_burnt {
            errors.validate_number("energy", energy, Some(0), Some(10000));
        }

        if let Some(notes) = notes {
            errors.validate_string("name", &notes, Some(3), Some(10000), None);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
