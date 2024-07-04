use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::exercise_plan::model::ExercisePlanQuery;
use crate::training_plan::weekday::WeekDay;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkoutPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutPlanQuery {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    //
    pub sequence: Option<i32>,
    pub weekday: Option<WeekDay>,
    pub training_plan_id: Option<Uuid>,
    pub training_plan_name: Option<String>,
    pub training_plan_slug: Option<String>,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub exercise_plans: Vec<ExercisePlanQuery>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutPlanInput {
    pub user_id: Uuid,
    pub name: String,
}
