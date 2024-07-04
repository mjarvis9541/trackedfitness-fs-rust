use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::set::model::{SetQuery, SetQueryWithPrevious};

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

impl ExerciseQueryWithPrevious {
    pub fn get_title(&self) -> String {
        format!("{}. {}", self.order, self.movement_name)
    }
    // pub fn get_detail_href(&self) -> String {
    //     format!(
    //         "/users/{}/workouts/{}/{}/{}",
    //         self.username, self.date, self.workout_id, self.exercise_id,
    //     )
    // }

    /// We should really do this on the backend... but reducing fronmtend code for now
    /// Get the weight of the last time in the sets vector.
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
