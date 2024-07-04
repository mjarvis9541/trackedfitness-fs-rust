use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MovementBase {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub muscle_group_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Movement {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub muscle_group_id: Uuid,
    pub muscle_group_name: String,
    pub muscle_group_slug: String,
    pub created_by: String,
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MovementWithLatestWeight {
    pub movement_id: Uuid,
    pub movement_name: String,
    pub movement_slug: String,
    pub muscle_group_name: String,
    pub muscle_group_slug: String,
    pub latest_workout_date: NaiveDate,
    pub latest_exercise_weight: Decimal,
    pub latest_exercise_sets: i64,
    pub latest_exercise_reps: i64,
}
