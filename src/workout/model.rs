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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkoutDaySummary {
    pub username: String,
    pub date: NaiveDate,
    pub total_workouts: i64,
    pub total_exercises: i64,
    pub total_sets: i64,
    pub total_reps: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutWeek {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub username: String,
    pub workouts: Vec<WorkoutQuery>,
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
pub struct WorkoutDayQuery {
    pub user_id: Uuid,
    pub workout_id: Uuid,
    pub workout_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub exercises: Vec<WorkoutDayExerciseQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutDayExerciseQuery {
    pub exercise_id: Uuid,
    pub movement_name: String,
    pub muscle_group_name: String,
    pub order: i32,
    pub set_count: i64,
    pub rep_count: i64,
    pub sets: Vec<WorkoutDaySetQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WorkoutDaySetQuery {
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

impl WorkoutDayQuery {
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

impl WorkoutDayExerciseQuery {
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
