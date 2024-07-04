use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::workout_plan::model::WorkoutPlanQuery;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrainingPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub duration_weeks: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TrainingPlanQuery {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub duration_weeks: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    // nfd
    pub workout_count: i64,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub workout_plans: Vec<WorkoutPlanQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrainingPlanInput {
    pub name: String,
    pub duration_weeks: i32,
    pub description: Option<String>,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct TrainingPlanWorkoutPlan {
//     pub training_plan_id: Uuid,
//     pub workout_plan_id: Uuid,
//     pub sequence: i32,
//     pub weekday: i32,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: Option<DateTime<Utc>>,
//     pub created_by_id: Uuid,
//     pub updated_by_id: Option<Uuid>,
// }
