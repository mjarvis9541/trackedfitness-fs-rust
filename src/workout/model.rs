use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::exercise::model::ExerciseQuery;
use crate::util::datetime::DATE_FORMAT_SHORT;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkoutBase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutQuery {
    pub user_id: Uuid,
    pub workout_id: Uuid,
    pub workout_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub exercises: Vec<ExerciseQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutQueryWithPrevious {
    pub user_id: Uuid,
    pub workout_id: Uuid,
    pub workout_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub exercises: Vec<ExerciseQueryWithPrevious>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ExerciseQueryWithPrevious {
    pub exercise_id: Uuid,
    pub movement_name: String,
    pub muscle_group_name: String,
    pub order: i32,
    pub set_count: i64,
    pub rep_count: i64,
    pub sets: Vec<SetQueryWithPrevious>,
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
pub struct WorkoutWeek {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub username: String,
    pub workouts: Vec<WorkoutQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutDetail {
    pub user_id: Uuid,
    pub username: String,
    pub workout_id: Uuid,
    pub workout_date: NaiveDate,
    pub workout_created_at: DateTime<Utc>,
    pub exercise_id: Uuid,
    pub exercise_created_at: DateTime<Utc>,
    pub exercise_order: i32,
    pub movement_name: String,
    pub muscle_group_name: Option<String>,
    pub set_order: Option<i32>,
    pub set_id: Option<Uuid>,
    pub weight: Option<Decimal>,
    pub reps: Option<i32>,
    pub rest: Option<i32>,
    pub previous_workout_id: Option<Uuid>,
    pub previous_workout_date: Option<NaiveDate>,
    pub previous_exercise_id: Option<Uuid>,
    pub previous_set_id: Option<Uuid>,
    pub previous_weight: Option<Decimal>,
    pub previous_reps: Option<i32>,
    pub workout_exercise_count: Option<i64>,
    pub workout_set_count: Option<i64>,
    pub workout_rep_count: Option<i64>,
    pub exercise_set_count: Option<i64>,
    pub exercise_rep_count: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutDTO {
    pub workout_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub exercise_list: Vec<ExerciseDTO>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ExerciseDTO {
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub movement_name: String,
    pub muscle_group_name: String,
    pub set_list: Vec<SetDTO>,
    pub previous_set_list: Vec<SetDTO>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SetDTO {
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
    pub set_id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub weight: Decimal,
    pub reps: i32,
    pub rest: i32,
    pub order: i32,
}

impl WorkoutQueryWithPrevious {
    pub fn format_date(&self) -> String {
        self.workout_date.format(DATE_FORMAT_SHORT).to_string()
    }

    pub fn get_workout_detail_url(&self) -> String {
        format!(
            "/users/{}/workouts/{}/{}",
            self.username, self.workout_date, self.workout_id
        )
    }
    pub fn get_add_exercise_url(&self) -> String {
        format!(
            "/users/{}/workouts/{}/{}/add-exercise",
            self.username, self.workout_date, self.workout_id
        )
    }
    pub fn get_add_workout_plan_url(&self) -> String {
        format!(
            "/users/{}/workouts/{}/{}/add-workout-plan",
            self.username, self.workout_date, self.workout_id
        )
    }
}

impl ExerciseQueryWithPrevious {
    pub fn get_title(&self) -> String {
        format!("{}. {}", self.order, self.movement_name)
    }

    pub fn get_last_set_weight(&self) -> String {
        self.sets
            .last()
            .map(|last_set| format!("{:.2}", last_set.weight))
            .unwrap_or_default()
    }

    pub fn get_last_set_reps(&self) -> i32 {
        self.sets.last().map(|set| set.reps).unwrap_or_default()
    }

    pub fn get_next_set_order(&self) -> i64 {
        (self.set_count + 1) as i64
    }
}
