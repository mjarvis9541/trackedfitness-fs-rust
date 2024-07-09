use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::util::validation_error::ValidationError;

use super::activity_level::ActivityLevel;
use super::fitness_goal::FitnessGoal;
use super::model::ProfileQuery;
use super::sex::Sex;

impl ProfileQuery {
    pub fn validate(
        sex: &str,
        activity_level: &str,
        fitness_goal: &str,
        height: Decimal,
        weight: Decimal,
        date_of_birth: NaiveDate,
    ) -> Result<(), ValidationError> {
        let min_height = Decimal::from(50);
        let max_height = Decimal::from(250);
        let min_weight = Decimal::from(20);
        let max_weight = Decimal::from(500);

        let mut errors = ValidationError::new();
        errors.validate_choice("sex", sex, &Sex::all_variants());
        errors.validate_choice(
            "activity_level",
            activity_level,
            &ActivityLevel::all_variants(),
        );
        errors.validate_choice("fitness_goal", fitness_goal, &FitnessGoal::all_variants());

        errors.validate_decimal("height", height, Some(min_height), Some(max_height));
        errors.validate_decimal("weight", weight, Some(min_weight), Some(max_weight));
        errors.validate_date("date_of_birth", date_of_birth, 365 * 150, 0);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
