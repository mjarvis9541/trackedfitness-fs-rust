use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::util::validation_error::ValidationError;

use super::model::Diet;

impl Diet {
    pub fn validate(date: NaiveDate, quantity: Decimal) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        errors.validate_date("date", date, 365, 365);

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
