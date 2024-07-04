use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetModel {
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub order: i32,
    pub weight: Decimal,
    pub reps: i32,
    pub rest: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SetQuery {
    pub id: Uuid,
    pub order: i32,
    pub weight: Decimal,
    pub reps: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SetQueryWithPrevious {
    pub set_id: Uuid,
    pub order: i32,
    pub weight: Decimal,
    pub reps: i32,
    pub rest: i32,
    pub previous_workout_id: Option<Uuid>,
    pub previous_workout_date: Option<NaiveDate>,
    pub previous_exercise_id: Option<Uuid>,
    pub previous_weight: Option<Decimal>,
    pub previous_reps: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SetToCreate {
    pub exercise_num: i32,
    pub weight: Decimal,
    pub reps: i32,
    pub rest: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MergedSetInputData {
    pub exercise_id: Uuid,
    pub exercise_num: i32,
    pub set_order: i32,
    pub weight: Decimal,
    pub reps: i32,
    pub rest: i32,
}
