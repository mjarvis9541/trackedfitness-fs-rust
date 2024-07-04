use chrono::NaiveDate;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diet_target::model::DietTarget;
use crate::error::{Error, Result};

#[derive(Debug)]
pub enum Variant {
    DietLog,
    DietTarget,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UserDaySummary {
    pub user_id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub weight: Decimal,
    pub energy: Decimal,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
    pub energy_per_kg: Decimal,
    pub protein_per_kg: Decimal,
    pub carbohydrate_per_kg: Decimal,
    pub fat_per_kg: Decimal,
    pub actual: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserWeekSummary {
    pub day_total_vec: Vec<UserDaySummary>,
    pub period_avg: UserDaySummary,
}

impl UserDaySummary {
    pub fn generate_main_link(&self, variant: Variant) -> String {
        match variant {
            Variant::DietLog => format!("/users/{}/diet/{}", self.username, self.date),
            Variant::DietTarget => {
                if self.actual {
                    format!("/users/{}/diet-targets/{}", self.username, self.date)
                } else {
                    format!(
                        "/users/{}/diet-targets/create?date={}",
                        self.username, self.date
                    )
                }
            }
        }
    }
}

impl UserDaySummary {
    pub fn add(&mut self, other: &UserDaySummary) {
        self.weight += other.weight;
        self.energy += other.energy;
        self.fat += other.fat;
        self.saturates += other.saturates;
        self.carbohydrate += other.carbohydrate;
        self.sugars += other.sugars;
        self.fibre += other.fibre;
        self.protein += other.protein;
        self.salt += other.salt;
        self.protein_pct += other.protein_pct;
        self.carbohydrate_pct += other.carbohydrate_pct;
        self.fat_pct += other.fat_pct;
        self.energy_per_kg += other.energy_per_kg;
        self.protein_per_kg += other.protein_per_kg;
        self.carbohydrate_per_kg += other.carbohydrate_per_kg;
        self.fat_per_kg += other.fat_per_kg;
    }

    pub fn divide(&mut self, divisor: Decimal) -> Result<()> {
        if divisor == Decimal::zero() {
            return Err(Error::Other("Division error".into()));
        }

        self.weight /= divisor;
        self.energy /= divisor;
        self.fat /= divisor;
        self.saturates /= divisor;
        self.carbohydrate /= divisor;
        self.sugars /= divisor;
        self.fibre /= divisor;
        self.protein /= divisor;
        self.salt /= divisor;
        self.protein_pct /= divisor;
        self.carbohydrate_pct /= divisor;
        self.fat_pct /= divisor;
        self.energy_per_kg /= divisor;
        self.protein_per_kg /= divisor;
        self.carbohydrate_per_kg /= divisor;
        self.fat_per_kg /= divisor;

        Ok(())
    }
}

impl From<DietTarget> for UserDaySummary {
    fn from(diet_target: DietTarget) -> Self {
        UserDaySummary {
            user_id: diet_target.user_id,
            username: diet_target.username,
            date: diet_target.date,
            weight: diet_target.weight,
            energy: Decimal::from(diet_target.energy),
            fat: diet_target.fat,
            saturates: diet_target.saturates,
            carbohydrate: diet_target.carbohydrate,
            sugars: diet_target.sugars,
            fibre: diet_target.fibre,
            protein: diet_target.protein,
            salt: diet_target.salt,
            protein_pct: diet_target.protein_pct,
            carbohydrate_pct: diet_target.carbohydrate_pct,
            fat_pct: diet_target.fat_pct,
            energy_per_kg: diet_target.energy_per_kg,
            protein_per_kg: diet_target.protein_per_kg,
            carbohydrate_per_kg: diet_target.carbohydrate_per_kg,
            fat_per_kg: diet_target.fat_per_kg,
            actual: true,
        }
    }
}
