use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::auth::privacy_level::PrivacyLevel;
use crate::follower::status::FollowerStatus;
use crate::food::model::Nutrition;
use crate::user_block::model::UserBlockStatus;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserQuery {
    pub name: String,
    pub username: String,
    pub is_self: bool,
    pub can_view: bool,
    pub privacy_level: PrivacyLevel,
    pub blocked_status: UserBlockStatus,
    pub follower_status: FollowerStatus,
    pub follower_count: i64,
    pub following_count: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WeekSummary {
    pub username: String,
    pub date: NaiveDate,
    pub week_item: Vec<WeekSummaryItem>,
    pub energy_per_kg: Decimal,
    pub protein_per_kg: Decimal,
    pub carbohydrate_per_kg: Decimal,
    pub fat_per_kg: Decimal,
    pub week_avg_weight: Option<Decimal>,
    pub nutrition: Nutrition,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WeekSummaryItem {
    pub username: String,
    pub date: NaiveDate,
    pub energy_per_kg: Decimal,
    pub protein_per_kg: Decimal,
    pub carbohydrate_per_kg: Decimal,
    pub fat_per_kg: Decimal,
    pub latest_weight: Option<Decimal>,
    pub latest_weight_date: Option<NaiveDate>,
    pub nutrition: Nutrition,
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
