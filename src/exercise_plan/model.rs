use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExercisePlan {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub movement_id: Uuid,
    pub sequence: i32,
    pub weight: Decimal,
    pub sets: i32,
    pub reps: i32,
    pub rest: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ExercisePlanQuery {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub movement_id: Uuid,
    pub sequence: i32,
    pub weight: Decimal,
    pub sets: i32,
    pub reps: i32,
    pub rest: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub movement_name: String,
    pub workout_plan_name: String,
    pub workout_plan_slug: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExercisePlanInput {
    pub workout_plan_id: Uuid,
    pub movement_id: Uuid,
    pub sequence: i32,
    pub weight: Decimal,
    pub sets: i32,
    pub reps: i32,
    pub rest: i32,
}
