use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Movement {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub muscle_group_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MovementQuery {
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

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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

impl MovementQuery {
    pub fn get_muscle_group_href(&self) -> String {
        format!("/exercises/muscle-groups/{}", self.muscle_group_slug)
    }
}
