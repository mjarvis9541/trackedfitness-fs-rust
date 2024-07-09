use chrono::{Datelike, NaiveDate};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diet_target::model::DietTargetQuery;
use crate::error::{Error, Result};
use crate::util::datetime::get_week_start;

#[derive(Debug)]
pub enum Variant {
    DietLog,
    DietTarget,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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

impl From<DietTargetQuery> for UserDaySummary {
    fn from(diet_target: DietTargetQuery) -> Self {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MonthSummary {
    pub username: String,
    pub date: NaiveDate,
    pub energy: Decimal,
    pub protein: Decimal,
    pub fat: Decimal,
    pub carbohydrate: Decimal,
    pub week_avg_energy: Decimal,
    pub week_avg_protein: Decimal,
    pub week_avg_carbohydrate: Decimal,
    pub week_avg_fat: Decimal,
    pub workout_count: i64,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub week_total_workouts: i64,
    pub week_total_exercises: i64,
    pub week_total_sets: i64,
    pub week_total_reps: i64,
    pub progress_date: Option<NaiveDate>,
    pub weight: Option<Decimal>,
    pub energy_burnt: Option<i32>,
    pub week_avg_weight: Decimal,
    pub week_avg_energy_burnt: i64,
}

impl MonthSummary {
    pub fn get_calendar_date_title(&self) -> String {
        self.date.format("%d %b").to_string()
    }
    pub fn get_week_title(&self) -> String {
        format!(
            "Week {}, {}",
            self.date.iso_week().week(),
            self.date.format("%Y")
        )
    }
    pub fn get_week_href(&self) -> String {
        let monday = get_week_start(self.date);
        format!("/users/{}/week/{}", self.username, monday)
    }
    pub fn get_profile_href(&self) -> String {
        format!("/users/{}/{}", self.username, self.date)
    }
    pub fn get_diet_day_href(&self) -> String {
        format!("/users/{}/diet/{}", self.username, self.date)
    }
    pub fn get_workout_day_href(&self) -> String {
        format!("/users/{}/workouts/{}", self.username, self.date)
    }
    pub fn get_progress_detail_or_create_href(&self) -> String {
        if let Some(date) = self.progress_date {
            format!("/users/{}/progress/{}", self.username, date)
        } else {
            format!(
                "/users/{}/progress/create?date={}",
                self.username, self.date
            )
        }
    }
}
