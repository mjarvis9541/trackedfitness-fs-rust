use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::set::model::SetQuery;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExerciseModel {
    pub id: Uuid,
    pub order: i32,
    pub workout_id: Uuid,
    pub movement_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ExerciseQuery {
    pub id: Uuid,
    pub workout_id: Uuid,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    // nfd
    pub date: NaiveDate,
    pub user_id: Uuid,
    pub username: String,
    pub movement_name: String,
    pub movement_slug: String,
    pub muscle_group_name: String,
    pub muscle_group_slug: String,
    pub set_count: i64,
    pub rep_count: i64,
    pub sets: Vec<SetQuery>,
}
