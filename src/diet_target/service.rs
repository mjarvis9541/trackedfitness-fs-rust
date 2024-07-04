use chrono::NaiveDate;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::error::Result;
use crate::profile::fitness_goal::{FitnessGoal, TargetModifier};

use super::model::{DietTargetGramKg, DietTargetInput};

impl DietTargetInput {
    pub fn calculate_nutrients(
        modifier: TargetModifier,
        tdee: Decimal,
        user_id: Uuid,
        date: NaiveDate,
        weight: Decimal,
    ) -> DietTargetInput {
        let energy = tdee * modifier.energy_factor;
        let protein = energy * modifier.protein_pct / Decimal::from(4);
        let carbohydrate = energy * modifier.carbohydrate_pct / Decimal::from(4);
        let fat = energy * modifier.fat_pct / Decimal::from(9);
        let saturates = fat * modifier.saturates_pct;
        let sugars = energy * modifier.sugars_pct;
        let fibre = modifier.fibre;
        let salt = modifier.salt;
        let energy = energy.round().to_i32().unwrap_or_default();
        DietTargetInput {
            user_id,
            date,
            weight,
            energy,
            fat,
            saturates,
            carbohydrate,
            sugars,
            fibre,
            protein,
            salt,
        }
    }

    pub fn from_fitness_goal_and_tdee(
        user_id: Uuid,
        date: NaiveDate,
        weight: Decimal,
        fitness_goal: FitnessGoal,
        tdee: Decimal,
    ) -> DietTargetInput {
        let modifier: TargetModifier = fitness_goal.into();
        Self::calculate_nutrients(modifier, tdee, user_id, date, weight)
    }

    pub fn from_weight_and_grams_per_kg(
        user_id: Uuid,
        date: NaiveDate,
        weight: Decimal,
        protein_per_kg: Decimal,
        carbohydrate_per_kg: Decimal,
        fat_per_kg: Decimal,
    ) -> Result<DietTargetInput> {
        let protein = weight * protein_per_kg;
        let carbohydrate = weight * carbohydrate_per_kg;
        let fat = weight * fat_per_kg;

        let energy = (protein * Decimal::from(4))
            + (carbohydrate * Decimal::from(4))
            + (fat * Decimal::from(9));

        let saturates = fat * Decimal::new(35, 2);
        let sugars = energy * Decimal::new(3, 2);
        let fibre = Decimal::from(30);
        let salt = Decimal::from(6);

        let energy = energy.round().to_i32().unwrap_or_default();

        Ok(DietTargetInput {
            user_id,
            date,
            weight,
            energy,
            fat,
            saturates,
            carbohydrate,
            sugars,
            fibre,
            protein,
            salt,
        })
    }
}

impl From<DietTargetGramKg> for DietTargetInput {
    fn from(data: DietTargetGramKg) -> DietTargetInput {
        let protein = data.weight * data.protein_per_kg;
        let carbohydrate = data.weight * data.carbohydrate_per_kg;
        let fat = data.weight * data.fat_per_kg;

        let energy = (protein * Decimal::from(4))
            + (carbohydrate * Decimal::from(4))
            + (fat * Decimal::from(9));

        let saturates = fat * Decimal::new(35, 2);
        let sugars = energy * Decimal::new(3, 2);
        let fibre = Decimal::from(30);
        let salt = Decimal::from(6);

        let energy = energy.round().to_i32().unwrap_or_default();

        DietTargetInput {
            user_id: data.user_id,
            date: data.date,
            weight: data.weight,
            energy,
            fat,
            saturates,
            carbohydrate,
            sugars,
            fibre,
            protein,
            salt,
        }
    }
}
